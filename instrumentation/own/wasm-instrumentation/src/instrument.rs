use ast::*;
use ast::Instr::*;
use std::mem::discriminant;

pub fn identity(_: &mut Module) {}

pub fn add_trivial_type(module: &mut Module) {
    add_type(
        &mut module.sections,
        FuncType {
            params: Vec::new().into(),
            results: Vec::new().into(),
        },
    );
}

pub fn count_call_instructions(module: &mut Module) {
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
            init: Expr(vec![I32Const(0.into()), End]),
        });
    }
}

// TODO generalize with macro_rules! to other sections, variables: Section::Variant, elementtype (e.g. FuncType), indextype
fn add_type(sections: &mut Vec<Section>, func_type: FuncType) -> TypeIdx {
    let empty_section = Section::Type(Vec::new().into());
    let section_idx = sections.iter()
        .position(|section| discriminant(section) == discriminant(&empty_section));
    let section_idx = section_idx.unwrap_or_else(|| insert_section(sections, empty_section));

    let section_elements: &mut Vec<_> = if let &mut Section::Type(ref mut elements) = &mut sections[section_idx] {
        &mut elements.content.value
    } else {
        unreachable!("we just found or inserted this section above, why has it not the expected type?")
    };

    let new_element_idx = section_elements.len();
    section_elements.push(func_type);
    TypeIdx(new_element_idx.into())
}

/// returns the index where the section was inserted
fn insert_section(sections: &mut Vec<Section>, to_insert: Section) -> usize {
    let insert_index = sections.iter()
        .position(|section| section > &to_insert)
        .unwrap_or(sections.len());
    sections.insert(insert_index, to_insert);
    insert_index
}

//// TODO I don't know if this generic approach works, so for now lets only provide some specialized methods above
//fn update_or_insert(sections: &mut Vec<Section>, to_insert: Section) {
//    // always insert custom sections at the back and DO NOT merge with other custom sections
//    if let Section::Custom(_) = to_insert {
//        sections.push(to_insert);
//        return;
//    }
//
//
//    // insert at appropriate index (e.g., Type section must come before all other non-custom sections)
//    let insert_index = sections
//        .position(|section| section > &to_insert)
//        .unwrap_or(sections.len());
//    sections.insert(insert_index, to_insert);
//
//    // merge adjacent sections of the same type together
//    // except for Start, replace the value there
////    sections.windows(2)
//}
//
//fn merge_section(a: Section, b: Section) -> Section {}

// TODO implement actual instrumentations:
// - call instruction counting
// - counting which function gets called how often
// - BB counting
// ...
