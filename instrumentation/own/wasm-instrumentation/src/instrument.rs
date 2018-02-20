use ast::highlevel::*;
use ast::lowlevel::FunctionType;

pub fn identity(_: &mut Module) {}

pub fn add_empty_function(module: &mut Module) {
    module.functions.push(Function {
        type_: FunctionType(vec![], vec![]),
        import: None,
        code: Some(Code {
            locals: vec![],
            body: vec![
                Instr::End
            ],
        }),
        export: None,
    })
}

// FIXME
//pub fn count_calls(module: &mut Module) {
//    let counter: GlobalIdx = add_global(&mut module.sections,
//                                        Global {
//                                            type_: GlobalType(ValType::I32, Mutability::Mut),
//                                            init: Expr(vec![I32Const(0.into()), End]),
//                                        });
//
//    let _getter: FunctionIdx = add_function_with_type(
//        &mut module.sections,
//        FunctionType {
//            params: Vec::new().into(),
//            results: vec![ValType::I32].into(),
//        },
//        Vec::new(),
//        Expr(vec![
//            GetGlobal(counter),
//            End
//        ]));
//
//    let _increment: FunctionIdx = add_function_with_type(
//        &mut module.sections,
//        FunctionType {
//            params: Vec::new().into(),
//            results: Vec::new().into(),
//        },
//        Vec::new(),
//        Expr(vec![
//            GetGlobal(counter),
//            I32Const(1.into()),
//            I32Add,
//            SetGlobal(counter),
//            End
//        ]));
//}
//
//// TODO implement actual instrumentations:
//// - call instruction counting
//// - counting which function gets called how often
//// - BB counting
//// ...
//
///// adds an element to the corresponding section if the section exists,
///// otherwise creates the section with no elements and inserts it then.
//macro_rules! add_section_element {
//    ($function_name: ident, $SectionVariant: ident, $ElementType: ty) => {
//        fn $function_name(sections: &mut Vec<Section>, element: $ElementType) {
//            let empty_section = Section::$SectionVariant(Vec::new().into());
//            let section_idx = sections.iter()
//                .position(|section| discriminant(section) == discriminant(&empty_section));
//            let section_idx = section_idx.unwrap_or_else(|| insert_section(sections, empty_section));
//
//            let section_elements: &mut Vec<_> = if let &mut Section::$SectionVariant(ref mut elements) = &mut sections[section_idx] {
//                &mut elements.content.value
//            } else {
//                unreachable!("we just found or inserted this section, why has it not the expected type?")
//            };
//
//            section_elements.push(element);
//        }
//    };
//    // FIXME can I somehow refactor out the repetition of the fn body!?
//    ($function_name: ident, $SectionVariant: ident, $ElementType: ty, $ElementIdxType: ident) => {
//        fn $function_name(sections: &mut Vec<Section>, element: $ElementType) -> $ElementIdxType {
//            let empty_section = Section::$SectionVariant(Vec::new().into());
//            let section_idx = sections.iter()
//                .position(|section| discriminant(section) == discriminant(&empty_section));
//            let section_idx = section_idx.unwrap_or_else(|| insert_section(sections, empty_section));
//
//            let section_elements: &mut Vec<_> = if let &mut Section::$SectionVariant(ref mut elements) = &mut sections[section_idx] {
//                &mut elements.content.value
//            } else {
//                unreachable!("we just found or inserted this section, why has it not the expected type?")
//            };
//
//            let new_element_idx = section_elements.len();
//            section_elements.push(element);
//            $ElementIdxType(new_element_idx.into())
//        }
//    };
//}
//
//add_section_element!(add_type, Type, FuncType, TypeIdx);
//add_section_element!(add_function, Function, TypeIdx, FunctionIdx);
//add_section_element!(add_code, Code, WithSize<Function>, FunctionIdx);
//add_section_element!(add_global, Global, Global, GlobalIdx);
//add_section_element!(add_export, Export, Export);
//// FIXME indices of imports (tables, memory, globals, but most importantly functions) count towards the
//// index space, in particular BEFORE the functions, globals etc. defined inside the module!
//// this means that
//// 1. FIXME the returned indices of add_function and add_global are wrong and need to be offset by the imports inside the module
//// 2. FIXME calling add_import invalidates ALL indices into functions etc., since their offset might change
//// -> FIXME we need a high level AST format!!
//// -> FIXME we cannot use static/const indices, but must take the import indices into account, i.e. get idx as a function from module?
//add_section_element!(add_import, Import, Import);
//
///// convenience instead of calling add_type, add_function, add_code manually
//fn add_function_with_type(sections: &mut Vec<Section>, ty: FunctionType, locals: Vec<Locals>, body: Expr) -> FunctionIdx {
//    let ty_idx = add_type(sections, ty);
//    let func_idx = add_function(sections, ty_idx);
//    let code_idx = add_code(sections, Function { locals: locals.into(), body }.into());
//    assert_eq!(func_idx, code_idx,
//               "function type and code should have same index, were Type and Code sections of the same size beforehand?");
//    func_idx
//}
//
///// returns the index where the section was inserted
//fn insert_section(sections: &mut Vec<Section>, to_insert: Section) -> usize {
//    let insert_index = sections.iter()
//        .position(|section| section > &to_insert)
//        .unwrap_or(sections.len());
//    sections.insert(insert_index, to_insert);
//    insert_index
//}