#[macro_use]
extern crate neon;

extern crate wasabi;
extern crate wasm;
use wasabi::config::EnabledHooks;
use wasabi::config::HighLevelHook;
use wasabi::instrument::add_hooks;
use wasm::ast::highlevel::Module;
use std::{fs, path::PathBuf};
use std::str::FromStr;

use neon::prelude::*;

pub struct Wasabi {
    input_file: String,
    output_dir: String,
    enabled_hooks: EnabledHooks,
}
declare_types! {
    pub class JsWasabi for Wasabi {
        init(mut cx) {
            let input_file: Handle<JsString> = cx.argument::<JsString>(0)?;
            let output_dir: Handle<JsString> = cx.argument::<JsString>(1)?;
            let enabled_hooks: EnabledHooks = match cx.argument_opt(2) {
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
                input_file: input_file.value(),
                output_dir: output_dir.value(),
                enabled_hooks,
            })
        }

        method get(mut cx) {
            let attr: String = cx.argument::<JsString>(0)?.value();

            let this = cx.this();
            match &attr[..] {
                "input_file" => {
                    let input_file = {
                        let guard = cx.lock();
                        let wasabi = this.borrow(&guard);
                        wasabi.input_file.clone()
                    };
                    Ok(cx.string(input_file).upcast())
                },
                "output_dir" => {
                    let output_dir = {
                        let guard = cx.lock();
                        let wasabi = this.borrow(&guard);
                        wasabi.output_dir.clone()
                    };
                    Ok(cx.string(output_dir).upcast())
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
                "input_file" => {
                    let input_file: String = cx.argument::<JsString>(1)?.value();
                    {
                        let guard = cx.lock();
                        let mut wasabi = this.borrow_mut(&guard);
                        wasabi.input_file = input_file;
                    }
                    Ok(cx.boolean(true).upcast())
                },
                "output_dir" => {
                    let output_dir: String = cx.argument::<JsString>(1)?.value();
                    {
                        let guard = cx.lock();
                        let mut wasabi = this.borrow_mut(&guard);
                        wasabi.output_dir = output_dir;
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
            let (input_file, output_dir, enabled_hooks) = {
                let guard = cx.lock();
                let wasabi = this.borrow(&guard);
                (wasabi.input_file.clone(), wasabi.output_dir.clone(), wasabi.enabled_hooks.0.clone())
            };
            let input_file = PathBuf::from(&input_file);
            let output_dir = PathBuf::from(&output_dir);
            let mut output_file_stem = output_dir.clone();
            let input_filename_no_ext = input_file.file_stem().unwrap();
            output_file_stem.push(input_filename_no_ext);
            let output_file_wasm = output_file_stem.with_extension("wasm");
            let output_file_js = output_file_stem.with_extension("wasabi.js");
            
            let enabled_hooks = EnabledHooks(enabled_hooks);

            // instrument Wasm and generate JavaScript
            let mut module = Module::from_file(input_file.clone()).unwrap();
            let js = add_hooks(&mut module, &enabled_hooks).unwrap();

            // write output files
            fs::create_dir_all(output_dir).unwrap();
            module.to_file(output_file_wasm).unwrap();
            fs::write(output_file_js, js).unwrap();
            Ok(cx.boolean(true).upcast())
        }
    }
}

register_module!(mut cx, {
    cx.export_class::<JsWasabi>("Wasabi")?;
    Ok(())
});
