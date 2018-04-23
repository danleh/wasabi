use ast::highlevel::{Instr, Instr::*, InstrGroup};
use ast::ValType::{self, *};

impl Instr {
    /// "generate" quick and dirty the low-level JavaScript hook function from an instruction
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
            (InstrGroup::Unary { input_ty, result_ty }, instr) => format!(
                "{}: function (func, instr, {}, {}) {{
    unary({{func, instr}}, \"{}\", {}, {});
}},",
                instr_name,
                arg("input", input_ty), arg("result", result_ty),
                instr_name,
                long("input", input_ty), long("result", result_ty)),
            (InstrGroup::Binary { first_ty, second_ty, result_ty }, instr) => format!(
                "{}: function (func, instr, {}, {}, {}) {{
    binary({{func, instr}}, \"{}\", {}, {}, {});
}},",
                instr_name,
                arg("first", first_ty), arg("second", second_ty), arg("result", result_ty),
                instr_name,
                long("first", first_ty), long("second", second_ty), long("result", result_ty)),
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
            },
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
