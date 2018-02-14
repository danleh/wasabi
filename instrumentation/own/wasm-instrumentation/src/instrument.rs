use ast::*;
use ast::Instr::*;
use std::mem::discriminant;

pub fn identity(_: &mut Module) {}

pub fn add_trivial_type(module: &mut Module) {
    add_type(&mut module.sections,
             FuncType {
                 params: Vec::new().into(),
                 results: Vec::new().into(),
             },
    );
}

pub fn count_calls(module: &mut Module) {
    let bla = add_global(&mut module.sections,
               Global {
                   type_: GlobalType(ValType::I32, Mut::Var),
                   init: Expr(vec![I32Const(0.into()), End]),
               }
    );
}

// TODO implement actual instrumentations:
// - call instruction counting
// - counting which function gets called how often
// - BB counting
// ...

macro_rules! add_section_element {
    ($function_name: ident, $SectionVariant: ident, $ElementType: ty, $ElementIdxType: ident) => {
        /// adds an element to the corresponding section if the section exists,
        /// otherwise creates the section with no elements and inserts it then.
        fn $function_name(sections: &mut Vec<Section>, element: $ElementType) -> $ElementIdxType {
            let empty_section = Section::$SectionVariant(Vec::new().into());
            let section_idx = sections.iter()
                .position(|section| discriminant(section) == discriminant(&empty_section));
            let section_idx = section_idx.unwrap_or_else(|| insert_section(sections, empty_section));

            let section_elements: &mut Vec<_> = if let &mut Section::$SectionVariant(ref mut elements) = &mut sections[section_idx] {
                &mut elements.content.value
            } else {
                unreachable!("we just found or inserted this section, why has it not the expected type?")
            };

            let new_element_idx = section_elements.len();
            section_elements.push(element);
            $ElementIdxType(new_element_idx.into())
        }
    }
}

add_section_element!(add_type, Type, FuncType, TypeIdx);
add_section_element!(add_global, Global, Global, GlobalIdx);

/// returns the index where the section was inserted
fn insert_section(sections: &mut Vec<Section>, to_insert: Section) -> usize {
    let insert_index = sections.iter()
        .position(|section| section > &to_insert)
        .unwrap_or(sections.len());
    sections.insert(insert_index, to_insert);
    insert_index
}