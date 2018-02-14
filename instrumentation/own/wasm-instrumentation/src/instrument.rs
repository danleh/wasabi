use ast::*;

pub fn identity(_: &mut Module) {}

pub fn add_trivial_function_type(module: &mut Module) {
    for mut section in module.sections.iter_mut() {
        if let Section::Type(ref mut types) = *section {
            types.content.push(FuncType { params: Vec::new().into(), results: Vec::new().into() });
        }
    }
}

pub fn count_call_instructions(module: &mut Module) {
    // TODO add impl on Vec<Section> with update_or_insert() fn
    // - always inserts Custom sections at the back
    // - for all other sections, inserts at the appropriate place
    // - and merges if there already is a section of that type (what to do with Start section? overwrite?)

    let mut vec = module.sections.iter_mut().filter_map(|section| match section {
        &mut Section::Global(ref mut globals) => {
            Some(&mut globals.content.value)
        }
        _ => None,
    }).last();
    // FIXME below probably doesnt work because cannot borrow temporary value :/
    // unwrap_or(&module.sections.push(Global(Vec::new().into()))

    if let Some(ref mut globals) = vec {
        globals.push(Global {
            type_: GlobalType(ValType::I32, Mut::Var),
            init: Expr(vec![Instr::I32Const(0.into())]),
        });
    }

//    let section_to_insert = Section::Global(Vec::new().into());
//
//    let insert_index = module.sections.iter()
//        .position(|section| section > &section_to_insert)
//        .unwrap_or(module.sections.len());
//
//    module.sections.insert(insert_index, section_to_insert);

    // TODO make sure some sections are not appearing twice, e.g., Global
    // TODO to fix this: add merging of sections
}

// TODO implement actual instrumentations:
// - call instruction counting
// - counting which function gets called how often
// - BB counting
// ...
