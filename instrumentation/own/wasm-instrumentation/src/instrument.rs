use ast::FuncType;
use ast::Module;
use ast::Section;

pub fn identity(module: Module) -> Module {
    module
}

pub fn add_trivial_function_type(mut module: Module) -> Module {
    for mut section in module.sections.iter_mut() {
        if let Section::Type(ref mut types) = *section {
            types.content.push(FuncType { params: Vec::new().into(), results: Vec::new().into() });
        }
    }
    module
}

// TODO implement actual instrumentations:
// - call instruction counting
// - counting which function gets called how often
// - BB counting
// ...
