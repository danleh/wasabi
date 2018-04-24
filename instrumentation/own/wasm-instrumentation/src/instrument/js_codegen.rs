use ast::highlevel::{Instr, Instr::*, InstrGroup};
use ast::ValType::{self, *};
use serde_json;
use super::static_info::ModuleInfo;

pub fn js_codegen(module_info: ModuleInfo, on_demand_hooks: &[String]) -> String {
    format!(r#"/*
 * Auto-generated from WASM module to-analyze.
 * DO NOT EDIT.
 */

const moduleInfo = {};

const lowlevelHooks = {{
{}{}
}};
"#,
            serde_json::to_string_pretty(&module_info).unwrap(),
            r#"    // trivial
    nop: function (func, instr) {
        nop({func, instr});
    },
    unreachable: function (func, instr) {
        unreachable({func, instr});
    },

    // memory
    memory_size: function (func, instr, currentSizePages) {
        memory_size({func, instr}, currentSizePages);
    },
    memory_grow: function (func, instr, byPages, previousSizePages) {
        memory_grow({func, instr}, byPages, previousSizePages);
    },

    // begin/ends
    begin_function: function (func, instr) {
        begin({func, instr}, "function");
    },
    end_function: function (func, instr) {
        end({func, instr}, "function", {func, instr: -1});
    },
    begin_block: function (func, instr) {
        begin({func, instr}, "block");
    },
    end_block: function (func, instr, begin_instr) {
        end({func, instr}, "block", {func, instr: begin_instr});
    },
    begin_loop: function (func, instr) {
        begin({func, instr}, "loop");
    },
    end_loop: function (func, instr, begin_instr) {
        end({func, instr}, "loop", {func, instr: begin_instr});
    },
    begin_if: function (func, instr) {
        begin({func, instr}, "if");
    },
    end_if: function (func, instr, begin_instr) {
        end({func, instr}, "if", {func, instr: begin_instr});
    },
    begin_else: function (func, instr) {
        begin({func, instr}, "else");
    },
    end_else: function (func, instr, begin_instr) {
        end({func, instr}, "else", {func, instr: begin_instr});
    },

    // branches/if condition
    if_: function (func, instr, condition) {
        if_({func, instr}, condition === 1);
    },
    br: function (func, instr, target_label, target_instr) {
        br({func, instr}, {label: target_label, location: {func, instr: target_instr}});
    },
    br_if: function (func, instr, target_label, target_instr, condition) {
        br_if({func, instr}, {label: target_label, location: {func, instr: target_instr}}, condition === 1);
    },
    br_table: function (func, instr, br_table_info_idx, table_idx) {
        br_table({
            func,
            instr
        }, moduleInfo.br_tables[br_table_info_idx].table, moduleInfo.br_tables[br_table_info_idx].default, table_idx);
    },

    // generated:
    "#,
            on_demand_hooks.iter().flat_map(|s| s.split("\n")).collect::<Vec<&str>>().join("\n    ")
    )
}

/// "generate" quick and dirty the low-level JavaScript hook function from an instruction
impl Instr {
    pub fn to_js_hook(&self) -> String {
        let instr_name = self.to_instr_name();
        match (self.group(), self) {
            (InstrGroup::Const(ty), instr) => format!(
                "{}: function (func, instr, {}) {{
    const_({{func, instr}}, {});
}},",
                instr_name,
                arg("v", ty), long("v", ty)
            ),
            (InstrGroup::Numeric { ref input_tys, ref result_tys }, instr) if input_tys.len() == 1 => format!(
                "{}: function (func, instr, {}, {}) {{
    unary({{func, instr}}, \"{}\", {}, {});
}},",
                instr_name,
                arg("input", input_tys[0]), arg("result", result_tys[0]),
                instr_name,
                long("input", input_tys[0]), long("result", result_tys[0])),
            (InstrGroup::Numeric { ref input_tys, ref result_tys }, instr) if input_tys.len() == 2 => format!(
                "{}: function (func, instr, {}, {}, {}) {{
    binary({{func, instr}}, \"{}\", {}, {}, {});
}},",
                instr_name,
                arg("first", input_tys[0]), arg("second", input_tys[1]), arg("result", result_tys[0]),
                instr_name,
                long("first", input_tys[0]), long("second", input_tys[1]), long("result", result_tys[0])),
            (InstrGroup::MemoryLoad(ty, _), instr) => format!(
                "{}: function (func, instr, offset, align, addr, {}) {{
    load({{func, instr}}, \"{}\", {{addr, offset, align}}, {});
}},",
                instr_name,
                arg("v", ty),
                instr_name,
                long("v", ty)),
            (InstrGroup::MemoryStore(ty, _), instr) => format!(
                "{}: function (func, instr, offset, align, addr, {}) {{
    store({{func, instr}}, \"{}\", {{addr, offset, align}}, {});
}},",
                instr_name,
                arg("v", ty),
                instr_name,
                long("v", ty)),
            _ => unimplemented!("cannot generate JS hook code for instruction {}", instr_name)
        }
    }

    pub fn to_poly_js_hook(&self, tys: &[ValType]) -> String {
        let hook_name = append_mangled_tys(self.to_instr_name(), tys);
        match *self {
            Return => {
                let return_hook = format!("{}: function(func, instr{}) {{
    return_({{func, instr}}, [{}]);
}},",
                                          hook_name,
                                          tys.iter().enumerate().map(|(i, ty)| format!(", {}", arg(&("result".to_string() + &i.to_string()), *ty))).collect::<String>(),
                                          tys.iter().enumerate().map(|(i, ty)| long(&("result".to_string() + &i.to_string()), *ty)).collect::<Vec<String>>().join(", "),
                );
                // FIXME dirty hack: add also post call (call_result) hooks here, since they are based on the same type information
                // FIXME because of replace() the hook name is "call_result_" instead of more natural "call_result"
                // FIXME no difference between call_result and call_indirect_result
                return_hook.clone() + "\n" + &return_hook.replace("return", "call_result")
            }
            // TODO rename to call_pre and the call_result -> call_post
            Call(_) => format!("{}: function(func, instr, targetFunc{}) {{
    call_({{func, instr}}, targetFunc, false, [{}]);
}},",
                               hook_name,
                               tys.iter().enumerate().map(|(i, ty)| format!(", {}", arg(&("arg".to_string() + &i.to_string()), *ty))).collect::<String>(),
                               tys.iter().enumerate().map(|(i, ty)| long(&("arg".to_string() + &i.to_string()), *ty)).collect::<Vec<String>>().join(", "),
            ),
            CallIndirect(_, _) => format!("{}: function(func, instr, targetTableIdx{}) {{
    call_({{func, instr}}, resolveTableIdx(targetTableIdx), true, [{}]);
}},",
                                          hook_name,
                                          tys.iter().enumerate().map(|(i, ty)| format!(", {}", arg(&("arg".to_string() + &i.to_string()), *ty))).collect::<String>(),
                                          tys.iter().enumerate().map(|(i, ty)| long(&("arg".to_string() + &i.to_string()), *ty)).collect::<Vec<String>>().join(", "),
            ),
            Drop => format!("{}: function(func, instr, {}) {{
    drop({{func, instr}}, {});
}},",
                            hook_name,
                            arg("v", tys[0]),
                            long("v", tys[0])
            ),
            Select => format!("{}: function(func, instr, condition, {}, {}) {{
    select({{func, instr}}, condition === 1, {}, {});
}},",
                              hook_name,
                              arg("first", tys[0]), arg("second", tys[1]),
                              long("first", tys[0]), long("second", tys[1]),
            ),
            GetLocal(_) => format!("{}: function(func, instr, index, {}) {{
    local({{func, instr}}, \"get\", index, {});
}},",
                                   hook_name,
                                   arg("v", tys[0]),
                                   long("v", tys[0])
            ),
            SetLocal(_) => format!("{}: function(func, instr, index, {}) {{
    local({{func, instr}}, \"set\", index, {});
}},",
                                   hook_name,
                                   arg("v", tys[0]),
                                   long("v", tys[0])
            ),
            TeeLocal(_) => format!("{}: function(func, instr, index, {}) {{
    local({{func, instr}}, \"tee\", index, {});
}},",
                                   hook_name,
                                   arg("v", tys[0]),
                                   long("v", tys[0])
            ),
            GetGlobal(_) => format!("{}: function(func, instr, index, {}) {{
    global({{func, instr}}, \"get\", index, {});
}},",
                                    hook_name,
                                    arg("v", tys[0]),
                                    long("v", tys[0])
            ),
            SetGlobal(_) => format!("{}: function(func, instr, index, {}) {{
    global({{func, instr}}, \"set\", index, {});
}},",
                                    hook_name,
                                    arg("v", tys[0]),
                                    long("v", tys[0])
            ),
            _ => unimplemented!("cannot generate JS hook code for instruction {}", self.to_instr_name())
        }
    }
}


/* helpers */

/// e.g. "call" + [I32, F32] -> "call_i32_f32"
pub fn append_mangled_tys(prefix: String, tys: &[ValType]) -> String {
    prefix + "_" + &tys.iter().map(|ty| ty.to_string()).collect::<Vec<_>>().join("_")
}

fn arg(name: &str, ty: ValType) -> String {
    match ty {
        I64 => name.to_string() + "_low, " + name + "_high",
        _ => name.to_string()
    }
}

fn long(name: &str, ty: ValType) -> String {
    match ty {
        I64 => format!("new Long({})", arg(name, ty)),
        _ => name.to_string()
    }
}
