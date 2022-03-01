use std::str::FromStr;

use lazy_static::lazy_static;
use walkdir::WalkDir;

use crate::highlevel::LoadOp::*;
use crate::highlevel::NumericOp::*;
use crate::highlevel::StoreOp::*;
use crate::highlevel::FunctionType;
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
                        type_: FunctionType::default(),
                        body: Body(Vec::new()), 
                        export: Vec::new()
                    },
                    Function {
                        name: Func::Idx(1),
                        type_: FunctionType::default(),
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
                        type_: FunctionType::default(),
                        body: Body(Vec::new()), 
                        export: vec!["name1".to_string(), "name2".to_string()],                        
                    },
                    Function {
                        name: Func::Idx(1),
                        type_: FunctionType::default(),
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
                        type_: FunctionType::default(),
                        body: Body(Vec::new()), 
                        export: Vec::new()
                    },
                    Function {
                        name: Func::Idx(1),
                        type_: FunctionType::new(&[ValType::I32], &[ValType::F64]),
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
                type_: FunctionType::default(),
                body: Body(Vec::new()), 
                export: Vec::new()
            },
            "func f0 () -> () @label0: {}",
            "empty function"
        ),
        (
            Function {
                name: Func::Idx(1),
                type_: FunctionType::new(&[ValType::I32], &[ValType::F64]),
                body: Body(Vec::new()), 
                export: Vec::new()
            },
            "func f1 (p0: i32) -> (r0: f64) @label0: {}",
            "empty function with types"
        ),
        (
            Function {
                name: Func::Idx(1),
                type_: FunctionType::new(&[ValType::I32], &[ValType::F64]),
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
                type_: FunctionType::new(&[ValType::I32], &[ValType::F64]),
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
    assert_eq!(Ok(Func::Named("bla".to_string().into())), "bla".parse());

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
    const WIMPL_TEST_INPUTS_DIR: &str = "tests/wimpl/wimplify_expected/";
    for entry in WalkDir::new(&WIMPL_TEST_INPUTS_DIR) {
        let wimpl_path = entry.unwrap().path().to_owned();

        // Find all files, where a <name>.wimpl file and a <name>.wasm file are next to each other.
        if let Some("wimpl") = wimpl_path.extension().and_then(|os_str| os_str.to_str()) {
            let wasm_path = wimpl_path.with_extension("wasm");
            if wasm_path.exists() {

                println!("\t{}", wimpl_path.display());
                
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

#[test]
fn wimplify_should_not_crash_on_realistic_files() {
    const WASM_TEST_INPUTS_DIR: &str = "tests/wasm/";
    for entry in WalkDir::new(&WASM_TEST_INPUTS_DIR) {
        let path = entry.unwrap().path().to_owned();

        if let Some("wasm") = path.extension().and_then(|os_str| os_str.to_str()) {
            if std::fs::metadata(&path).unwrap().is_file() {

                println!("\t{}", path.display());
                let wimpl_module = wimplify(&path);
                assert!(wimpl_module.is_ok(), "couldn't wimplify testcase: {}", path.display());

            }
        }
    }
}