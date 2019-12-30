#[macro_use]
extern crate neon;

extern crate wasabi;
extern crate wasm;
use wasabi::config::EnabledHooks;
use wasabi::config::HighLevelHook;
use wasabi::instrument::add_hooks;
use wasm::ast::highlevel::Module;
use std::str::FromStr;

use neon::prelude::*;

pub struct Wasabi {
    binary_file: Vec<u8>,
    enabled_hooks: EnabledHooks,
}
declare_types! {
    pub class JsWasabi for Wasabi {
        init(mut cx) {
            let binary_file_buffer: Handle<JsBuffer> = cx.argument(0)?;
            let mut binary_file = Vec::new();
            cx.borrow(&binary_file_buffer, |data| {
                let slice = data.as_slice::<u8>();
                for (_i, item) in slice.iter().enumerate() {
                    binary_file.push(*item)
                }
            });
            let enabled_hooks: EnabledHooks = match cx.argument_opt(1) {
                Some(arg) => {
                    let enabled_hooks_array = arg.downcast::<JsArray>().or_throw(&mut cx)?;
                    let enabled_hooks_vec: Vec<Handle<JsValue>> = enabled_hooks_array.to_vec(&mut cx)?;
                    EnabledHooks(enabled_hooks_vec.iter().map(|hook| {
                        let hook_str = hook
                            .downcast::<JsString>()
                            .unwrap()
                            .value();
                        HighLevelHook::from_str(hook_str.as_str()).unwrap()
                    }).collect())
                },
                None => EnabledHooks::all(),
            };
            Ok(Wasabi {
                enabled_hooks,
                binary_file,
            })
        }

        method get(mut cx) {
            let attr: String = cx.argument::<JsString>(0)?.value();

            let this = cx.this();
            match &attr[..] {
                "binary_file" => {
                    let binary_file = {
                        let guard = cx.lock();
                        let wasabi = this.borrow(&guard);
                        wasabi.binary_file.clone()
                    };
                    let mut js_buffer = JsBuffer::new(&mut cx, binary_file.len() as u32)?;
                    cx.borrow_mut(&mut js_buffer, |data| {
                        let slice = data.as_mut_slice::<u8>();
                        for (i, item) in binary_file.iter().enumerate() {
                            slice[i] = *item;
                        }
                    });
                    Ok(js_buffer.upcast())
                },
                "enabled_hooks" => {
                    let enabled_hooks = {
                        let guard = cx.lock();
                        let wasabi = this.borrow(&guard);
                        wasabi.enabled_hooks.0.clone()
                    };
                    // Create the JS array
                    let js_array = JsArray::new(&mut cx, enabled_hooks.len() as u32);
                    for (i, obj) in enabled_hooks.iter().enumerate() {
                        let js_string = cx.string(obj);
                        js_array.set(&mut cx, i as u32, js_string).unwrap();
                    }
                    Ok(js_array.upcast())
                },
                _ => cx.throw_type_error("property does not exist")
            }
        }

        method set(mut cx) {
            let attr: String = cx.argument::<JsString>(0)?.value();
            let mut this = cx.this();
            match &attr[..] {
                "binary_file" => {
                    let binary_file_buffer: Handle<JsBuffer> = cx.argument::<JsBuffer>(1)?;
                    let mut binary_file = Vec::new();
                    cx.borrow(&binary_file_buffer, |data| {
                        let slice = data.as_slice::<u8>();
                        for (_i, item) in slice.iter().enumerate() {
                            binary_file.push(*item)
                        }
                    });
                    {
                        let guard = cx.lock();
                        let mut wasabi = this.borrow_mut(&guard);
                        wasabi.binary_file = binary_file;
                    }
                    Ok(cx.boolean(true).upcast())
                },
                "enabled_hooks" => {
                    let enabled_hooks_array: Handle<JsArray> = cx.argument::<JsArray>(1)?;
                    let enabled_hooks_vec: Vec<Handle<JsValue>> = enabled_hooks_array.to_vec(&mut cx)?;
                    let enabled_hooks = EnabledHooks(enabled_hooks_vec.iter().map(|hook| {
                        let hook_str = hook
                            .downcast::<JsString>()
                            .unwrap()
                            .value();
                        HighLevelHook::from_str(hook_str.as_str()).unwrap()
                    }).collect());
                    {
                        let guard = cx.lock();
                        let mut wasabi = this.borrow_mut(&guard);
                        wasabi.enabled_hooks = enabled_hooks;
                    }
                    Ok(cx.boolean(true).upcast())
                },
                _ => cx.throw_type_error("property does not exist")
            }
        }

        method exec(mut cx) {
            let this = cx.this();
            let (mut binary_file, enabled_hooks) = {
                let guard = cx.lock();
                let wasabi = this.borrow(&guard);
                (wasabi.binary_file.clone(), wasabi.enabled_hooks.0.clone())
            };
            
            let enabled_hooks = EnabledHooks(enabled_hooks);

            // instrument Wasm and generate JavaScript
            let mut module = Module::from_buf(&mut binary_file).unwrap();
            let js = add_hooks(&mut module, &enabled_hooks).unwrap();
            let js = cx.string(&js);

            let mut buffer: Vec<u8> = Vec::new();
            module.to_buf(&mut buffer).unwrap();
            
            // Create the JS buffer
            let mut js_buffer = JsBuffer::new(&mut cx, buffer.len() as u32)?;
            cx.borrow_mut(&mut js_buffer, |data| {
                let slice = data.as_mut_slice::<u8>();
                for (i, obj) in buffer.iter().enumerate() {
                    slice[i] = *obj;
                }
            });

            // Create result object
            let result = JsObject::new(&mut cx);
            result.set(&mut cx, "wasm", js_buffer)?;
            result.set(&mut cx, "js", js)?;
            Ok(result.upcast())
        }
    }
}

register_module!(mut cx, {
    cx.export_class::<JsWasabi>("Wasabi")?;
    Ok(())
});
