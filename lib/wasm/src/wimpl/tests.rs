use std::str::FromStr;

use lazy_static::lazy_static;

use crate::highlevel::LoadOp::*;
use crate::highlevel::NumericOp::*;
use crate::highlevel::StoreOp::*;
use crate::FunctionType;
use crate::Memarg;
use crate::Val::*;
use crate::ValType;

use crate::wimpl::*;
use crate::wimpl::Stmt::*;
use crate::wimpl::Expr::*;
use crate::wimpl::Var::*;

use crate::wimpl;
use crate::wimpls;

use crate::wimpl::wimplify::*;  

lazy_static! {
    static ref WIMPL_MODULE_SYNTAX_TESTCASES: Vec<(Module, &'static str, &'static str)> = vec![
        (
            Module {
                functions: Vec::new(),
                globals: Vec::new(),
                tables: Vec::new(),
            },
            "module {
}
",
            "empty module"
        ),
        (
            Module {
                functions: vec![
                    Function {
                        name: Func::Idx(0),
                        type_: FunctionType { params: Vec::new().into_boxed_slice(), results: Vec::new().into_boxed_slice() },
                        body: Body(Vec::new()), 
                        export: Vec::new()
                    },
                    Function {
                        name: Func::Idx(1),
                        type_: FunctionType { params: Vec::new().into_boxed_slice(), results: Vec::new().into_boxed_slice() },
                        body: Body(Vec::new()), 
                        export: Vec::new()
                    },
                ],
                globals: Vec::new(),
                tables: Vec::new(),
            },
            "module {
  func f0 () -> () @label0: {}
  func f1 () -> () @label0: {}
}
",
            "module with several empty fuctions"
        ),(
            Module {
                functions: vec![
                    Function {
                        name: Func::Idx(0),
                        type_: FunctionType { params: Vec::new().into_boxed_slice(), results: Vec::new().into_boxed_slice() },
                        body: Body(Vec::new()), 
                        export: vec!["name1".to_string(), "name2".to_string()],                        
                    },
                    Function {
                        name: Func::Idx(1),
                        type_: FunctionType { params: Vec::new().into_boxed_slice(), results: Vec::new().into_boxed_slice() },
                        body: Body(Vec::new()), 
                        export: vec!["name3".to_string()],
                    },
                ],
                globals: Vec::new(),
                tables: Vec::new(),
            },
            r#"module {
  export "name1", "name2"
  func f0 () -> () @label0: {}
  export "name3"
  func f1 () -> () @label0: {}
}
"#,
            "module with several empty fuctions"
        ),
        (
            Module {
                functions: vec![
                    Function {
                        name: Func::Idx(0),
                        type_: FunctionType { params: Vec::new().into_boxed_slice(), results: Vec::new().into_boxed_slice() },
                        body: Body(Vec::new()), 
                        export: Vec::new()
                    },
                    Function {
                        name: Func::Idx(1),
                        type_: FunctionType { params: vec![ValType::I32].into_boxed_slice(), results:vec![ValType::F64].into_boxed_slice() },
                        body: Body(vec![
                            Assign {
                                lhs: Stack(0),
                                rhs: Const(I32(3)),
                                type_: ValType::I32,
                            },
                            Assign {
                                lhs: Stack(1),
                                rhs: Const(I32(4)),
                                type_: ValType::I32,
                            },
                        ]), 
                        export: Vec::new()
                    },
                ],
                globals: Vec::new(),
                tables: Vec::new(),
            },
            "module {
  func f0 () -> () @label0: {}
  func f1 (p0: i32) -> (r0: f64) @label0: {
    s0: i32 = i32.const 3
    s1: i32 = i32.const 4
  }
}
",
            "module with several empty fuctions"
        ),
    ];

    static ref WIMPL_FUNCTION_SYNTAX_TESTCASES: Vec<(Function, &'static str, &'static str)> = vec![
        (
            Function {
                name: Func::Idx(0),
                type_: FunctionType { params: Vec::new().into_boxed_slice(), results: Vec::new().into_boxed_slice() },
                body: Body(Vec::new()), 
                export: Vec::new()
            },
            "func f0 () -> () @label0: {}",
            "empty function"
        ),
        (
            Function {
                name: Func::Idx(1),
                type_: FunctionType { params: vec![ValType::I32].into_boxed_slice(), results:vec![ValType::F64].into_boxed_slice() },
                body: Body(Vec::new()), 
                export: Vec::new()
            },
            "func f1 (p0: i32) -> (r0: f64) @label0: {}",
            "empty function with types"
        ),
        (
            Function {
                name: Func::Idx(1),
                type_: FunctionType { params: vec![ValType::I32].into_boxed_slice(), results:vec![ValType::F64].into_boxed_slice() },
                body: Body(vec![
                    Assign {
                        lhs: Stack(0),
                        rhs: Const(I32(3)),
                        type_: ValType::I32,
                    }
                ]), 
                export: Vec::new()
            },
            "func f1 (p0: i32) -> (r0: f64) @label0: { s0: i32 = i32.const 3 }",
            "function with i32.const in body"
        ),
        (
            Function {
                name: Func::Idx(1),
                type_: FunctionType { params: vec![ValType::I32].into_boxed_slice(), results:vec![ValType::F64].into_boxed_slice() },
                body: Body(vec![
                    Assign {
                        lhs: Stack(0),
                        rhs: Const(I32(3)),
                        type_: ValType::I32,
                    },
                    Assign {
                        lhs: Stack(1),
                        rhs: Const(I32(4)),
                        type_: ValType::I32,
                    },
                ]), 
                export: Vec::new()
            },
            "func f1 (p0: i32) -> (r0: f64) @label0: {
  s0: i32 = i32.const 3
  s1: i32 = i32.const 4
}",
            "function with multiple statements"
        ),
    ];

    /// Pairs of Wimpl AST with concrete syntax, and optionally a comment what is
    /// "special" about this testcase. This is used for testing both parsing and
    /// pretty-printing of Wimpl, just in different directions.
    ///
    /// For these examples, the concrete syntax is in the "canonical pretty"
    /// format, i.e., with "standard" whitespace.
    static ref WIMPL_CANONICAL_SYNTAX_TESTCASES: Vec<(Stmt, &'static str, &'static str)> = vec![
        (Unreachable, "unreachable", ""),
        (
            Assign {
                lhs: Stack(0),
                type_: ValType::I32,
                rhs: MemorySize
            },
            "s0: i32 = memory.size",
            ""
        ),
        (
            Assign {
                lhs: Global(0),
                type_: ValType::I32,
                rhs: VarRef(Local(0))
            },
            "g0: i32 = l0",
            "var ref on rhs"
        ),
        (
            Assign {
                lhs: Stack(0),
                rhs: Const(I32(1337)),
                type_: ValType::I32,
            },
            "s0: i32 = i32.const 1337",
            ""
        ),
        (
            Assign {
                lhs: Stack(1),
                rhs: Numeric {
                    op: I32Add,
                    args: vec![Stack(2), Stack(3)],
                },
                type_: ValType::I32,
            },
            "s1: i32 = i32.add(s2, s3)",
            ""
        ),
        (
            Assign {
                lhs: Stack(1),
                rhs: Load {
                    op: I32Load,
                    memarg: Memarg::default(I32Load),
                    addr: Stack(0),
                },
                type_: ValType::I32,
            },
            "s1: i32 = i32.load(s0)",
            ""
        ),
        (
            Store {
                op: I64Store8,
                value: Stack(1),
                addr: Stack(2),
                memarg: Memarg {
                    offset: 0,
                    alignment_exp: 4,
                },
            },
            "i64.store8 align=16 (s2) (s1)",
            "memory operation with non-default alignment"
        ),
        (
            Br {
                target: Label(0),
            },
            "br @label0",
            ""
        ),
        (
            Expr(Call {
                func: Func::Idx(7),
                args: Vec::new(),
            }),
            "call f7 ()",
            "call argument list is always printed, even if empty"
        ),
        (
            Assign {
                lhs: Stack(1),
                rhs: CallIndirect {
                    type_: FunctionType::new(&[ValType::I32], &[ValType::I32]),
                    table_idx: Stack(0),
                    args: vec![Stack(2), Stack(3)],
                },
                type_: ValType::I32,
            },
            "s1: i32 = call_indirect [i32] -> [i32] (s0) (s2, s3)",
            ""
        ),
        (
            Block {
                end_label: Label(0),
                body: Body (vec![]),
            },
            "@label0: block {}",
            "empty block"
        ),
        (
            Block {
                end_label: Label(1),
                body: Body(vec![
                    Assign{
                        lhs: Stack(1),
                        type_: ValType::I32,
                        rhs: VarRef(Stack(0)),
                    }]),
            },
            "@label1: block { s1: i32 = s0 }",
            "block with a single instruction, on one line"
        ),
        (
            Loop {
                begin_label: Label(0),
                body: Body (vec![
                    Br { target: Label(0) },
                    Unreachable
                ]),
            },
            r"@label0: loop {
  br @label0
  unreachable
}",
            "loop with multiple instructions, indentation"
        ),
        (
            If {
                    condition: Stack(0),
                    if_body: Body (
                        vec![Br {
                            target: Label(0),
                        }]),
                    else_body: None,
            },
            "if (s0) { br @label0 }",
            "if + br (which is our form of br_if)"
        ),
        (
            Switch {
                    index: Stack(0),
                    cases: vec![
                        Body(vec![Unreachable]),
                        Body(vec![Expr(MemorySize), Br { target: Label(1) }]),
                    ],
                    default: Body(vec![]),
            },
            r"switch (s0) {
  case 0: { unreachable }
  case 1: {
    memory.size
    br @label1
  }
  default: {}
}",
            "switch with multiple cases, some on a single line, others not"
        ),
        (
            Loop {
                begin_label: Label(1),
                body: Body(vec![
                    Block {
                        end_label: Label(2),
                        body: Body(vec![
                            Assign{
                                lhs: Stack(0),
                                type_: ValType::F64,
                                rhs: VarRef(Stack(1))
                            },
                            Unreachable,
                        ])
                    },
                    Br { target: Label(1) },
                ])
            },
            r"@label1: loop {
  @label2: block {
    s0: f64 = s1
    unreachable
  }
  br @label1
}",
        "nested and multi-line loop/block")
    ];

    /// The following examples are NOT in the canonical text format, e.g.,
    /// because they contain too little or too much whitespace.
    /// They are only used for testing parsing, not pretty-printing.
    static ref WIMPL_ALTERNATIVE_SYNTAX_TESTCASES: Vec<(Stmt, &'static str, &'static str)> = vec![
        (
            Assign {
                lhs: Stack(1),
                type_: ValType::I32,
                rhs: MemoryGrow { pages: Stack(0) },
            },
            "s1: i32 = memory.grow ( s0 )",
            "extra space around arguments"),
        (
            Expr(Call {
                func: Func::Idx(2),
                args: vec![Stack(2), Stack(3)],
            }),
            "call f2 ( s2, s3 )",
            "extra space around call arguments"
        ),
        (
            Assign{
                lhs: Stack(1),
                rhs: CallIndirect {
                    type_: FunctionType::new(&[ValType::I32], &[ValType::I32]),
                    table_idx: Stack(0),
                    args: vec![],
                },
                type_: ValType::I32,
            },
            "s1: i32 = call_indirect [  i32  ] ->[i32] (s0) ()",
            "non-standard spacing around function type"
        ),
        (
            Assign{
                lhs: Stack(1),
                rhs: Numeric {
                    op: I32Add,
                    args: vec![Stack(2), Stack(3)],
                },
                type_: ValType::I32,
            },
            "s1: i32 = i32.add (s2,s3)",
            "space before arguments, no space after comma"
        ),
        (
            Store {
                op: I64Store8,
                value: Stack(1),
                addr: Stack(2),
                memarg: Memarg {
                    offset: 0,
                    alignment_exp: 4,
                },
            },
            "i64.store8 align=16(s2)(s1)",
            "minimal spacing around arguments"
        ),
        (
            Block {
                end_label: Label(0),
                body: Body(vec![]),
            },
            "@label0:block{}",
            "minimal space in block"
        ),
        (
            Block {
                    end_label: Label(2),
                    body: Body(vec![
                        Assign {
                            lhs: Stack(1),
                            rhs: VarRef(Stack(0)),
                            type_: ValType::I32,
                        },
                        Expr(VarRef(Stack(1)))
                    ]),
            },
            "@label2: block { s1: i32 = s0 s1 }",
            "weird but valid parse: expression statement with only a variable reference"
        )
    ];
}

#[test]
fn pretty_print_stmt() {
    for (i, (wimpl, text, msg)) in WIMPL_CANONICAL_SYNTAX_TESTCASES.iter().enumerate() {
        assert_eq!(&wimpl.to_string(), text, "\ntest #{}\n{}", i, msg);
    }
}

#[test]
fn pretty_print_function() {
    for (i, (wimpl, text, msg)) in WIMPL_FUNCTION_SYNTAX_TESTCASES.iter().enumerate() {
        assert_eq!(&wimpl.to_string(), text, "\ntest #{}\n{}", i, msg);
    }
}

#[test]
fn pretty_print_module() {
    for (i, (wimpl, text, msg)) in WIMPL_MODULE_SYNTAX_TESTCASES.iter().enumerate() {
        assert_eq!(&wimpl.to_string(), text, "\ntest #{}\n{}", i, msg);
    }
}

#[test]
fn parse_var() {
    assert_eq!(Ok(Stack(0)), "s0".parse());
    assert_eq!(Ok(Global(42)), "g42".parse());
    assert_eq!(Ok(BlockResult(1)), "b1".parse());
    assert_eq!(Ok(Param(2)), "p2".parse());
    assert_eq!(Ok(Return(0)), "r0".parse());

    // Negative tests:
    assert!(
        "".parse::<Var>().is_err(),
        "empty is not allowed"
    );
    assert!(
        " s0 \n ".parse::<Var>().is_err(),
        "whitespace is not allowed"
    );
    assert!(
        "sABC".parse::<Var>().is_err(),
        "characters instead of number"
    );
    assert!(
        "x123".parse::<Var>().is_err(),
        "invalid variable type/prefix"
    );
}

#[test]
fn parse_func_id() {
    assert_eq!(Ok(Func::Idx(13)), "f13".parse());
    assert_eq!(Ok(Func::Named("bla".to_string())), "bla".parse());

    // Negative tests:
    assert!(
        "".parse::<Var>().is_err(),
        "empty is not allowed"
    );
    assert!(
        "123\n ".parse::<Func>().is_err(),
        "only number for function name is not allowed"
    );
}

#[test]
fn parse_expr() {
    assert_eq!(Ok(MemorySize), "memory.size".parse());
    assert_eq!(Ok(MemoryGrow { pages: Local(0) }), "memory.grow (l0)".parse());
    assert_eq!(Ok(VarRef(Global(1))), "g1".parse());
    assert_eq!(Ok(Numeric {
        op: I32Add,
        args: vec![Stack(0), Local(1)]
    }), "i32.add(s0, l1)".parse());
    // More complex expressions are tested in the statements.
}

#[test]
fn parse_stmt() {
    let parse_testcases = WIMPL_CANONICAL_SYNTAX_TESTCASES
        .iter()
        .chain(WIMPL_ALTERNATIVE_SYNTAX_TESTCASES.iter());
    for (i, (wimpl, text, msg)) in parse_testcases.enumerate() {
        let parsed = Stmt::from_str(text);
        match parsed {
            Err(err) => panic!(
                "\ntest #{} could not be parsed\ninput: `{}`\nerr: `{}`\n{}",
                i, text, err, msg
            ),
            Ok(parsed) => {
                assert_eq!(&parsed, wimpl, "\ntest #{}\ninput: `{}`\n{}", i, text, msg)
            }
        };
    }
}

#[test]
fn parse_pretty_print_roundtrips() {
    // For the text inputs in the canonical format, parsing then pretty-printing
    // the AST should round-trip, i.e., pretty(parse(text)) == text.
    for (i, (_, text, msg)) in WIMPL_CANONICAL_SYNTAX_TESTCASES.iter().enumerate() {
        let parsed = Stmt::from_str(text).unwrap();
        let pretty = parsed.to_string();
        assert_eq!(
            text, &pretty,
            "\nleft is input, right is pretty-printed\ntest #{}\n{}",
            i, msg
        );
    }

    // For the text inputs in non-canonical format, first parse and pretty-print
    // to canonicalize, then parse should be equal to non-canonicalized, i.e.
    // parse(pretty(parse(text))) == parse(text).
    for (i, (_, text, msg)) in WIMPL_CANONICAL_SYNTAX_TESTCASES.iter().enumerate() {
        let parsed = Stmt::from_str(text).unwrap();
        let pretty = parsed.to_string();
        let parsed_pretty = Stmt::from_str(&pretty).unwrap();
        assert_eq!(parsed, parsed_pretty, "\ntest #{}\n{}", i, msg);
    }
}

#[test]
fn parse_wimpl_text_file() {
    let instrs = Stmt::from_text_file("tests/wimpl/syntax.wimpl");
    assert!(instrs.is_ok());
}

#[test]
fn macros_should_compile_and_not_fail_at_runtime() {
    let _ = wimpl!(g0: f32 = f32.const 1.1);
    let _ = wimpl!(s2: i32 = i32.add (s0, s1));
    let _ = wimpl!(call_indirect [ ] ->[] (s1) ());

    // Tricky cases, because rustc lexes these tokens differently than we need to.
    let _ = wimpl!(s3: i32 = i32.load offset=3 (s0));
    let _ = wimpl!(@label0: block {});

    // Multiple instructions:
    let _ = wimpls! {};
    let _ = wimpls! {
        @label2: loop {
            s5: i32 = i32.const 3
            br @label2
        }
        l0: i64 = g0
    };
}

#[test]
fn wimplify_with_expected_output() {
    use walkdir::WalkDir;
    
    const WIMPL_TEST_INPUTS_DIR: &str = "tests/wimpl/wimplify_expected/";
    for entry in WalkDir::new(&WIMPL_TEST_INPUTS_DIR) {
        let wimpl_path = entry.unwrap().path().to_owned();

        // Find all files, where a <name>.wimpl file and a <name>.wasm file are next to each other.
        if let Some("wimpl") = wimpl_path.extension().and_then(|os_str| os_str.to_str()) {
            let wasm_path = wimpl_path.with_extension("wasm");
            if wasm_path.exists() {

                println!("{}", wimpl_path.display());
                
                let wimpl_module = wimplify(wasm_path).unwrap();

                // Every wimpl file contains only a sequence of statements, not a whole module.
                // Compare the first function from the .wasm binary, with all instructions of the
                // .wimpl text file.
                let actual = wimpl_module.functions[0].clone().body;
                let expected = Body(Stmt::from_text_file(&wimpl_path).unwrap());

                assert_eq!(actual, expected, "testcase: {}\nexpected Wimpl: {}\nproduced Wimpl: {}\n", wimpl_path.display(), expected, actual);
                
            }
        }
    }
}

// #[test]
// fn wimplify_should_not_crash() {
    
//     const WASM_TEST_INPUTS_DIR: &str = "tests/";

//     for path in wasm_files(WASM_TEST_INPUTS_DIR).unwrap() {
    
//     }
    
// }
// const WASM_TEST_INPUTS_DIR: &str = "tests/";

//     for path in wasm_files(WASM_TEST_INPUTS_DIR).unwrap() {


//hand written calculator programs

#[test]
fn calc() {
    wimplify("tests/wimpl-wasm-handwritten/calc/add.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn calc_dce() {
    wimplify("tests/wimpl-wasm-handwritten/calc-dce/add-dce.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn calc_virtual() {
    wimplify("tests/wimpl-wasm-handwritten/calc-virtual/add.wasm").expect("error while translating wasm file to wimpl");

}

//USENIX programs

#[test]
fn module_8c087e0290bb39f1e090() {
    wimplify("tests/wimpl-USENIX/8c087e0290bb39f1e090.module/8c087e0290bb39f1e090.module.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn annots() {
    wimplify("tests/wimpl-USENIX/annots/annots.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn module_bb9bb638551198cd3a42() {
    wimplify("tests/wimpl-USENIX/bb9bb638551198cd3a42.module/bb9bb638551198cd3a42.module.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn compiled_wasm() {
    wimplify("tests/wimpl-USENIX/compiled.wasm/compiled.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn module_dac34eee5ed4216c65b2() {
    wimplify("tests/wimpl-USENIX/dac34eee5ed4216c65b2.module/dac34eee5ed4216c65b2.module.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn imagequant_c970f() {
    wimplify("tests/wimpl-USENIX/imagequant.c970f/imagequant.c970f.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn mozjpeg_enc_93395() {
    wimplify("tests/wimpl-USENIX/mozjpeg_enc.93395/mozjpeg_enc.93395.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn optipng_4e77b() {
    wimplify("tests/wimpl-USENIX/optipng.4e77b/optipng.4e77b.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn rotate_4cdaa() {
    wimplify("tests/wimpl-USENIX/rotate.4cdaa/rotate.4cdaa.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn usenix_bin_acrobat_wasm() {
    wimplify("tests/wimpl-USENIX/USENIX_bin_acrobat.wasm/USENIX_bin_acrobat.wasm.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn webp_dec_fa0ab() {
    wimplify("tests/wimpl-USENIX/webp_dec.fa0ab/webp_dec.fa0ab.wasm").expect("error while translating wasm file to wimpl");

}

#[test]
fn webp_enc_ea665() {
    wimplify("tests/wimpl-USENIX/webp_enc.ea665/webp_enc.ea665.wasm").expect("error while translating wasm file to wimpl");

}

// filtered wasm binaries

#[test]
fn _07735b34f092d6e63c397dfb583b64ceca84c595d13c6912f8b0d414b0f01da9() {
    wimplify("tests/wimpl-filtered-binaries/07735b34f092d6e63c397dfb583b64ceca84c595d13c6912f8b0d414b0f01da9/07735b34f092d6e63c397dfb583b64ceca84c595d13c6912f8b0d414b0f01da9.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _14ee85873e07b6226d416a1fc3bfc2aeae9c44700eac316a04bb6d98b95b605c() {
    wimplify("tests/wimpl-filtered-binaries/14ee85873e07b6226d416a1fc3bfc2aeae9c44700eac316a04bb6d98b95b605c/14ee85873e07b6226d416a1fc3bfc2aeae9c44700eac316a04bb6d98b95b605c.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _1cbe05896a7233e4a5679d69b4fb4e04b3857b50b1a0fc0d1a34054ff40e39bb() {
    wimplify("tests/wimpl-filtered-binaries/1cbe05896a7233e4a5679d69b4fb4e04b3857b50b1a0fc0d1a34054ff40e39bb/1cbe05896a7233e4a5679d69b4fb4e04b3857b50b1a0fc0d1a34054ff40e39bb.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _1e9df781a23d49aaf1e10abc4db11cde4c31e07d9cee2568d723b27bbeff515b() {
    wimplify("tests/wimpl-filtered-binaries/1e9df781a23d49aaf1e10abc4db11cde4c31e07d9cee2568d723b27bbeff515b/1e9df781a23d49aaf1e10abc4db11cde4c31e07d9cee2568d723b27bbeff515b.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _2449e3cbacf8bc6fd02a09b5b2a0f7ad4555046f7afba480d29f4929d39e4b04() {
    wimplify("tests/wimpl-filtered-binaries/2449e3cbacf8bc6fd02a09b5b2a0f7ad4555046f7afba480d29f4929d39e4b04/2449e3cbacf8bc6fd02a09b5b2a0f7ad4555046f7afba480d29f4929d39e4b04.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _32e1e40f2bc99176f2ede6999af1681b9893fca63db2f87ef2d274cd43f1d3e3() {
    wimplify("tests/wimpl-filtered-binaries/32e1e40f2bc99176f2ede6999af1681b9893fca63db2f87ef2d274cd43f1d3e3/32e1e40f2bc99176f2ede6999af1681b9893fca63db2f87ef2d274cd43f1d3e3.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _381e5189553901c1649b5093758fee36b338ee7fcd211a20b1b6e6d374e53bce() {
    wimplify("tests/wimpl-filtered-binaries/381e5189553901c1649b5093758fee36b338ee7fcd211a20b1b6e6d374e53bce/381e5189553901c1649b5093758fee36b338ee7fcd211a20b1b6e6d374e53bce.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _3deb83bb20eccb638a2fbbe09ba323a472654552f8bcd93611e1d4ba20a67ea4() {
    wimplify("tests/wimpl-filtered-binaries/3deb83bb20eccb638a2fbbe09ba323a472654552f8bcd93611e1d4ba20a67ea4/3deb83bb20eccb638a2fbbe09ba323a472654552f8bcd93611e1d4ba20a67ea4.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _3f8cf6588c2ed1e7f92ef8f3c37cbb0a0294a9b2a0b330ecd087cff985b34689() {
    wimplify("tests/wimpl-filtered-binaries/3f8cf6588c2ed1e7f92ef8f3c37cbb0a0294a9b2a0b330ecd087cff985b34689/3f8cf6588c2ed1e7f92ef8f3c37cbb0a0294a9b2a0b330ecd087cff985b34689.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _419963e6d5166128b11ce6eb7138fe6b5c81694196882cba034f296d613d9d0f() {
    wimplify("tests/wimpl-filtered-binaries/419963e6d5166128b11ce6eb7138fe6b5c81694196882cba034f296d613d9d0f/419963e6d5166128b11ce6eb7138fe6b5c81694196882cba034f296d613d9d0f.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _4666a4a9c39036d84f70ebb3e2cb476cff2549377ced946618b293fc6552aae8() {
    wimplify("tests/wimpl-filtered-binaries/4666a4a9c39036d84f70ebb3e2cb476cff2549377ced946618b293fc6552aae8/4666a4a9c39036d84f70ebb3e2cb476cff2549377ced946618b293fc6552aae8.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _4696f2d4f93b20b80cb53d0ead51ebacefd3f407654878578374133e630a1fff() {
    wimplify("tests/wimpl-filtered-binaries/4696f2d4f93b20b80cb53d0ead51ebacefd3f407654878578374133e630a1fff/4696f2d4f93b20b80cb53d0ead51ebacefd3f407654878578374133e630a1fff.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _4ca2a66a0c64388ded652fc19aab816513782fcd86d57862abc35a0d25861314() {
    wimplify("tests/wimpl-filtered-binaries/4ca2a66a0c64388ded652fc19aab816513782fcd86d57862abc35a0d25861314/4ca2a66a0c64388ded652fc19aab816513782fcd86d57862abc35a0d25861314.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _524b1b048e588dc5a207e342503b454641f32ecafcb4d48d7d526bdeea6f5e98() {
    wimplify("tests/wimpl-filtered-binaries/524b1b048e588dc5a207e342503b454641f32ecafcb4d48d7d526bdeea6f5e98/524b1b048e588dc5a207e342503b454641f32ecafcb4d48d7d526bdeea6f5e98.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _57dee2a170d275e099b953a8fdafde6f5becea8a0a2202de68254ad74dfd6fd5() {
    wimplify("tests/wimpl-filtered-binaries/57dee2a170d275e099b953a8fdafde6f5becea8a0a2202de68254ad74dfd6fd5/57dee2a170d275e099b953a8fdafde6f5becea8a0a2202de68254ad74dfd6fd5.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _58fd82e10ee3f41aef7088281c2747eb4aa07300b4eefbc566080a074f6e9f3c() {
    wimplify("tests/wimpl-filtered-binaries/58fd82e10ee3f41aef7088281c2747eb4aa07300b4eefbc566080a074f6e9f3c/58fd82e10ee3f41aef7088281c2747eb4aa07300b4eefbc566080a074f6e9f3c.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _616140f09fb1466810df2b2bb70a5d4d692581d5c50985d58b66b23537ece6cb() {
    wimplify("tests/wimpl-filtered-binaries/616140f09fb1466810df2b2bb70a5d4d692581d5c50985d58b66b23537ece6cb/616140f09fb1466810df2b2bb70a5d4d692581d5c50985d58b66b23537ece6cb.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _6a0160516ec0012faf38d1a1e138806fc3085e956498e049276341e67eb63648() {
    wimplify("tests/wimpl-filtered-binaries/6a0160516ec0012faf38d1a1e138806fc3085e956498e049276341e67eb63648/6a0160516ec0012faf38d1a1e138806fc3085e956498e049276341e67eb63648.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _6c2ed8ebb8fe662fe7e0675e45dea2dbcdb387ce8367809390f58a1bcb63f3a8() {
    wimplify("tests/wimpl-filtered-binaries/6c2ed8ebb8fe662fe7e0675e45dea2dbcdb387ce8367809390f58a1bcb63f3a8/6c2ed8ebb8fe662fe7e0675e45dea2dbcdb387ce8367809390f58a1bcb63f3a8.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _7165c282edac43a4731b7aaae47e049de200dd1b5cc3c7710a5b57989927b394() {
    wimplify("tests/wimpl-filtered-binaries/7165c282edac43a4731b7aaae47e049de200dd1b5cc3c7710a5b57989927b394/7165c282edac43a4731b7aaae47e049de200dd1b5cc3c7710a5b57989927b394.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _77b3e5c5903371f52b3a829f889349d4ccee4f82fa2791325e5e4aea89efd793() {
    wimplify("tests/wimpl-filtered-binaries/77b3e5c5903371f52b3a829f889349d4ccee4f82fa2791325e5e4aea89efd793/77b3e5c5903371f52b3a829f889349d4ccee4f82fa2791325e5e4aea89efd793.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _787959bde2695ac32a0ee4bb92350f6568139c75f120012770fb3de012919736() {
    wimplify("tests/wimpl-filtered-binaries/787959bde2695ac32a0ee4bb92350f6568139c75f120012770fb3de012919736/787959bde2695ac32a0ee4bb92350f6568139c75f120012770fb3de012919736.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _835d0731f9ae86c0147196aeed12f6967e3886edd4d0b85637bb37a2f02b4875() {
    wimplify("tests/wimpl-filtered-binaries/835d0731f9ae86c0147196aeed12f6967e3886edd4d0b85637bb37a2f02b4875/835d0731f9ae86c0147196aeed12f6967e3886edd4d0b85637bb37a2f02b4875.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _88c0ee6c82e21d686b0ca9ab15c7fa6c551bd49dcfb6493d59a845ccd9cfc88e() {
    wimplify("tests/wimpl-filtered-binaries/88c0ee6c82e21d686b0ca9ab15c7fa6c551bd49dcfb6493d59a845ccd9cfc88e/88c0ee6c82e21d686b0ca9ab15c7fa6c551bd49dcfb6493d59a845ccd9cfc88e.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _8a5f2590830612d57d229c543645e64db599c1b2ea975b78a86c9ac5d6e5d88a() {
    wimplify("tests/wimpl-filtered-binaries/8a5f2590830612d57d229c543645e64db599c1b2ea975b78a86c9ac5d6e5d88a/8a5f2590830612d57d229c543645e64db599c1b2ea975b78a86c9ac5d6e5d88a.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _8ae683329370c6a0d5e5cd533ea34ae6fe17433ad0106524397448539dc0a14f() {
    wimplify("tests/wimpl-filtered-binaries/8ae683329370c6a0d5e5cd533ea34ae6fe17433ad0106524397448539dc0a14f/8ae683329370c6a0d5e5cd533ea34ae6fe17433ad0106524397448539dc0a14f.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _8ebf4e44c47b6b61d313bd2580bd788a1daa029541fe210cccfa13d1bb66cc89() {
    wimplify("tests/wimpl-filtered-binaries/8ebf4e44c47b6b61d313bd2580bd788a1daa029541fe210cccfa13d1bb66cc89/8ebf4e44c47b6b61d313bd2580bd788a1daa029541fe210cccfa13d1bb66cc89.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _9073aea62a25c574c19a69ed7232d6abf666cccb190e485a9860ce8fa244bd5b() {
    wimplify("tests/wimpl-filtered-binaries/9073aea62a25c574c19a69ed7232d6abf666cccb190e485a9860ce8fa244bd5b/9073aea62a25c574c19a69ed7232d6abf666cccb190e485a9860ce8fa244bd5b.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _9110face5f3ebd6d321619a8c5378c64e1ad159b0d4ce7fc31ee7b4702e013d8() {
    wimplify("tests/wimpl-filtered-binaries/9110face5f3ebd6d321619a8c5378c64e1ad159b0d4ce7fc31ee7b4702e013d8/9110face5f3ebd6d321619a8c5378c64e1ad159b0d4ce7fc31ee7b4702e013d8.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _921b6ab9805103b1bdca68f0e705cb80b499a24fce4e74943fd9e0b36ea0910c() {
    wimplify("tests/wimpl-filtered-binaries/921b6ab9805103b1bdca68f0e705cb80b499a24fce4e74943fd9e0b36ea0910c/921b6ab9805103b1bdca68f0e705cb80b499a24fce4e74943fd9e0b36ea0910c.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _92d3be911b9e7a6a5293c6ccedfcb734f5a35adba6ea7b9ced1a810e9716092f() {
    wimplify("tests/wimpl-filtered-binaries/92d3be911b9e7a6a5293c6ccedfcb734f5a35adba6ea7b9ced1a810e9716092f/92d3be911b9e7a6a5293c6ccedfcb734f5a35adba6ea7b9ced1a810e9716092f.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _9813df4ed1e42ea1cd0ec3b43d51a0beb80518980eb269c5bb35062710d4edee() {
    wimplify("tests/wimpl-filtered-binaries/9813df4ed1e42ea1cd0ec3b43d51a0beb80518980eb269c5bb35062710d4edee/9813df4ed1e42ea1cd0ec3b43d51a0beb80518980eb269c5bb35062710d4edee.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13() {
    wimplify("tests/wimpl-filtered-binaries/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13/a132c19bdeee909290fe971ba01b3c2d7f475eae25509766abd425a01bf1cc13.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _a8f75a78f2ef9f331c1d7e1327d90ea0c3a198e09d792c9bd7e0ca43d362f725() {
    wimplify("tests/wimpl-filtered-binaries/a8f75a78f2ef9f331c1d7e1327d90ea0c3a198e09d792c9bd7e0ca43d362f725/a8f75a78f2ef9f331c1d7e1327d90ea0c3a198e09d792c9bd7e0ca43d362f725.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _b6736dcdf2ff1eae4b54839fc3c25cef63ea4f3900acfed203c0bf692a771d60() {
    wimplify("tests/wimpl-filtered-binaries/b6736dcdf2ff1eae4b54839fc3c25cef63ea4f3900acfed203c0bf692a771d60/b6736dcdf2ff1eae4b54839fc3c25cef63ea4f3900acfed203c0bf692a771d60.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _bc3b3bf954993dc4914c3e924f4587b260680e8249d5f44f6be2eecae94082da() {
    wimplify("tests/wimpl-filtered-binaries/bc3b3bf954993dc4914c3e924f4587b260680e8249d5f44f6be2eecae94082da/bc3b3bf954993dc4914c3e924f4587b260680e8249d5f44f6be2eecae94082da.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _c0d83afa613ef8df9dd24c3b8737c8bdde493524b8ec0f7c5a928b7fe765fa73() {
    wimplify("tests/wimpl-filtered-binaries/c0d83afa613ef8df9dd24c3b8737c8bdde493524b8ec0f7c5a928b7fe765fa73/c0d83afa613ef8df9dd24c3b8737c8bdde493524b8ec0f7c5a928b7fe765fa73.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _c46e332b470498643fadd1d598b0285bd44c1f60204321ad4c01a5a3a5e22338() {
    wimplify("tests/wimpl-filtered-binaries/c46e332b470498643fadd1d598b0285bd44c1f60204321ad4c01a5a3a5e22338/c46e332b470498643fadd1d598b0285bd44c1f60204321ad4c01a5a3a5e22338.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _d25b99a719fef7cc681c79e8a77e17e87e6f7a3c423032f9b962f62c003dc38d() {
    wimplify("tests/wimpl-filtered-binaries/d25b99a719fef7cc681c79e8a77e17e87e6f7a3c423032f9b962f62c003dc38d/d25b99a719fef7cc681c79e8a77e17e87e6f7a3c423032f9b962f62c003dc38d.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _d49d001de11a69755e3b9014bcdc88c1ba77eeafc8b4e8db0f92fe31e8e6aee2() {
    wimplify("tests/wimpl-filtered-binaries/d49d001de11a69755e3b9014bcdc88c1ba77eeafc8b4e8db0f92fe31e8e6aee2/d49d001de11a69755e3b9014bcdc88c1ba77eeafc8b4e8db0f92fe31e8e6aee2.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _d70070b5a582d75eaa0d5896e56ca67a5399ab43c9e830791f1e2e8334404c90() {
    wimplify("tests/wimpl-filtered-binaries/d70070b5a582d75eaa0d5896e56ca67a5399ab43c9e830791f1e2e8334404c90/d70070b5a582d75eaa0d5896e56ca67a5399ab43c9e830791f1e2e8334404c90.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _e6b183e40f2671dc0f87e6d85070077e9bea18e7ad50cd7e7cd31e2a9ade937b() {
    wimplify("tests/wimpl-filtered-binaries/e6b183e40f2671dc0f87e6d85070077e9bea18e7ad50cd7e7cd31e2a9ade937b/e6b183e40f2671dc0f87e6d85070077e9bea18e7ad50cd7e7cd31e2a9ade937b.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _ec2393766f8f2a21803b4a062f1d71c184f6eae0be60f13703e3a43e5fa493b0() {
    wimplify("tests/wimpl-filtered-binaries/ec2393766f8f2a21803b4a062f1d71c184f6eae0be60f13703e3a43e5fa493b0/ec2393766f8f2a21803b4a062f1d71c184f6eae0be60f13703e3a43e5fa493b0.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _f2943fa8ae6133fb532fa9b036ba81342828b2e1e78b3ae47731619edcbca4dd() {
    wimplify("tests/wimpl-filtered-binaries/f2943fa8ae6133fb532fa9b036ba81342828b2e1e78b3ae47731619edcbca4dd/f2943fa8ae6133fb532fa9b036ba81342828b2e1e78b3ae47731619edcbca4dd.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _f4cd145be9df9b4b35e2ba3a95c6a6f0f5f8284a03de77c0e627a78d97fdaea2() {
    wimplify("tests/wimpl-filtered-binaries/f4cd145be9df9b4b35e2ba3a95c6a6f0f5f8284a03de77c0e627a78d97fdaea2/f4cd145be9df9b4b35e2ba3a95c6a6f0f5f8284a03de77c0e627a78d97fdaea2.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _fc762c3b4338c7d7a6bb31d478cfbe5717ebefb0e91d6d27b82a21fc169c7afe() {
    wimplify("tests/wimpl-filtered-binaries/fc762c3b4338c7d7a6bb31d478cfbe5717ebefb0e91d6d27b82a21fc169c7afe/fc762c3b4338c7d7a6bb31d478cfbe5717ebefb0e91d6d27b82a21fc169c7afe.wasm").expect("error while translating wasm file to wimpl");
}

#[test]
fn _fd6372aef6ff7d9ecffcc7f3d8d00963bebf39d68451c5ef36c039616ccbded3() {
    wimplify("tests/wimpl-filtered-binaries/fd6372aef6ff7d9ecffcc7f3d8d00963bebf39d68451c5ef36c039616ccbded3/fd6372aef6ff7d9ecffcc7f3d8d00963bebf39d68451c5ef36c039616ccbded3.wasm").expect("error while translating wasm file to wimpl");
}
