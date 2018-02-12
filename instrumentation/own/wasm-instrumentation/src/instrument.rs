use ast::FuncType;
use ast::Global;
use ast::Module;
use ast::Section;

pub fn identity(_: &mut Module) {}

pub fn add_trivial_function_type(module: &mut Module) {
    for mut section in module.sections.iter_mut() {
        if let Section::Type(ref mut types) = *section {
            types.content.push(FuncType { params: Vec::new().into(), results: Vec::new().into() });
        }
    }
}

pub fn count_call_instructions(module: &mut Module) {
    module.sections.insert(3, Section::Global(Vec::new().into()));

//    let globals: &mut Vec<Global> = module.sections.iter_mut()
//        .find(|ref section| {
//            if let &Section::Global(_) = section {
//                true
//            } else {
//                false
//            }
//        })

    // TODO make convenience function on modules that does

//    for mut section in module.sections.iter_mut() {}
//    module
}

// TODO implement actual instrumentations:
// - call instruction counting
// - counting which function gets called how often
// - BB counting
// ...
