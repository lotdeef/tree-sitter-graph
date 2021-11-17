// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, tree-sitter authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use tree_sitter_graph::ast::*;
use tree_sitter_graph::Context;
use tree_sitter_graph::Location;

#[test]
fn can_parse_blocks() {
    let mut ctx = Context::new();
    let source = r#"
        (function_definition
          name: (identifier) @cap1) @cap2
        {
          node loc1
          node @cap2.prop1
          edge @cap2.prop1 -> loc1
          attr (@cap2.prop1 -> loc1) precedence
          attr (@cap2.prop1) push = "str2", pop
          var @cap2.var1 = loc1
          set @cap2.var1 = loc1
        }
    "#;
    let mut file = File::new(tree_sitter_python::language());
    file.parse(&mut ctx, source).expect("Cannot parse file");

    let loc1 = ctx.add_identifier("loc1");
    let precedence = ctx.add_identifier("precedence");
    let pop = ctx.add_identifier("pop");
    let prop1 = ctx.add_identifier("prop1");
    let push = ctx.add_identifier("push");
    let cap2 = ctx.add_identifier("@cap2");
    let var1 = ctx.add_identifier("var1");

    let statements = file
        .stanzas
        .into_iter()
        .map(|s| s.statements)
        .collect::<Vec<_>>();
    assert_eq!(
        statements,
        vec![vec![
            CreateGraphNode {
                node: UnscopedVariable {
                    name: loc1,
                    location: Location { row: 4, column: 15 }
                }
                .into(),
                location: Location { row: 4, column: 10 }
            }
            .into(),
            CreateGraphNode {
                node: ScopedVariable {
                    scope: Box::new(
                        Capture {
                            index: 1,
                            name: cap2
                        }
                        .into()
                    ),
                    name: prop1,
                    location: Location { row: 5, column: 21 }
                }
                .into(),
                location: Location { row: 5, column: 10 },
            }
            .into(),
            CreateEdge {
                source: ScopedVariable {
                    scope: Box::new(
                        Capture {
                            index: 1,
                            name: cap2
                        }
                        .into()
                    ),
                    name: prop1,
                    location: Location { row: 6, column: 21 }
                }
                .into(),
                sink: UnscopedVariable {
                    name: loc1,
                    location: Location { row: 6, column: 30 },
                }
                .into(),
                location: Location { row: 6, column: 10 },
            }
            .into(),
            AddEdgeAttribute {
                source: ScopedVariable {
                    scope: Box::new(
                        Capture {
                            index: 1,
                            name: cap2
                        }
                        .into()
                    ),
                    name: prop1,
                    location: Location { row: 7, column: 22 },
                }
                .into(),
                sink: UnscopedVariable {
                    name: loc1.into(),
                    location: Location { row: 7, column: 31 },
                }
                .into(),
                attributes: vec![Attribute {
                    name: precedence,
                    value: Expression::TrueLiteral
                }],
                location: Location { row: 7, column: 10 },
            }
            .into(),
            AddGraphNodeAttribute {
                node: ScopedVariable {
                    scope: Box::new(
                        Capture {
                            index: 1,
                            name: cap2
                        }
                        .into()
                    ),
                    name: prop1,
                    location: Location { row: 8, column: 22 },
                }
                .into(),
                attributes: vec![
                    Attribute {
                        name: push,
                        value: String::from("str2").into(),
                    },
                    Attribute {
                        name: pop,
                        value: Expression::TrueLiteral
                    },
                ],
                location: Location { row: 8, column: 10 },
            }
            .into(),
            DeclareMutable {
                variable: ScopedVariable {
                    scope: Box::new(
                        Capture {
                            index: 1,
                            name: cap2
                        }
                        .into()
                    ),
                    name: var1,
                    location: Location { row: 9, column: 20 },
                }
                .into(),
                value: UnscopedVariable {
                    name: loc1,
                    location: Location { row: 9, column: 27 },
                }
                .into(),
                location: Location { row: 9, column: 10 },
            }
            .into(),
            Assign {
                variable: ScopedVariable {
                    scope: Box::new(
                        Capture {
                            index: 1,
                            name: cap2
                        }
                        .into()
                    ),
                    name: var1,
                    location: Location {
                        row: 10,
                        column: 20
                    },
                }
                .into(),
                value: UnscopedVariable {
                    name: loc1,
                    location: Location {
                        row: 10,
                        column: 27
                    },
                }
                .into(),
                location: Location {
                    row: 10,
                    column: 10
                },
            }
            .into(),
        ]]
    );
}

#[test]
fn can_parse_literals() {
    let mut ctx = Context::new();
    let source = r#"
        (identifier)
        {
          let f = #false
          let n = #null
          let t = #true
        }
    "#;
    let mut file = File::new(tree_sitter_python::language());
    file.parse(&mut ctx, source).expect("Cannot parse file");

    let f = ctx.add_identifier("f");
    let n = ctx.add_identifier("n");
    let t = ctx.add_identifier("t");

    let statements = file
        .stanzas
        .into_iter()
        .map(|s| s.statements)
        .collect::<Vec<_>>();
    assert_eq!(
        statements,
        vec![vec![
            DeclareImmutable {
                variable: UnscopedVariable {
                    name: f,
                    location: Location { row: 3, column: 14 }
                }
                .into(),
                value: Expression::FalseLiteral,
                location: Location { row: 3, column: 10 },
            }
            .into(),
            DeclareImmutable {
                variable: UnscopedVariable {
                    name: n,
                    location: Location { row: 4, column: 14 }
                }
                .into(),
                value: Expression::NullLiteral,
                location: Location { row: 4, column: 10 },
            }
            .into(),
            DeclareImmutable {
                variable: UnscopedVariable {
                    name: t,
                    location: Location { row: 5, column: 14 }
                }
                .into(),
                value: Expression::TrueLiteral,
                location: Location { row: 5, column: 10 },
            }
            .into(),
        ]]
    );
}

#[test]
fn can_parse_strings() {
    let mut ctx = Context::new();
    let source = r#"
        (identifier)
        {
          let loc1 = "\"abc,\ndef\\"
        }
    "#;
    let mut file = File::new(tree_sitter_python::language());
    file.parse(&mut ctx, source).expect("Cannot parse file");

    let loc1 = ctx.add_identifier("loc1");

    let statements = file
        .stanzas
        .into_iter()
        .map(|s| s.statements)
        .collect::<Vec<_>>();
    assert_eq!(
        statements,
        vec![vec![DeclareImmutable {
            variable: UnscopedVariable {
                name: loc1,
                location: Location { row: 3, column: 14 }
            }
            .into(),
            value: String::from("\"abc,\ndef\\").into(),
            location: Location { row: 3, column: 10 },
        }
        .into()]]
    );
}

#[test]
fn can_parse_lists() {
    let mut ctx = Context::new();
    let source = r#"
        (identifier)
        {
          let list1 = [1, 2, 3]
          let list2 = []
          let list3 = ["hello", "world",]
        }
    "#;
    let mut file = File::new(tree_sitter_python::language());
    file.parse(&mut ctx, source).expect("Cannot parse file");

    let list1 = ctx.add_identifier("list1");
    let list2 = ctx.add_identifier("list2");
    let list3 = ctx.add_identifier("list3");

    let statements = file
        .stanzas
        .into_iter()
        .map(|s| s.statements)
        .collect::<Vec<_>>();
    assert_eq!(
        statements,
        vec![vec![
            DeclareImmutable {
                variable: UnscopedVariable {
                    name: list1,
                    location: Location { row: 3, column: 14 }
                }
                .into(),
                value: ListComprehension {
                    elements: vec![
                        IntegerConstant { value: 1 }.into(),
                        IntegerConstant { value: 2 }.into(),
                        IntegerConstant { value: 3 }.into(),
                    ],
                }
                .into(),
                location: Location { row: 3, column: 10 },
            }
            .into(),
            DeclareImmutable {
                variable: UnscopedVariable {
                    name: list2,
                    location: Location { row: 4, column: 14 }
                }
                .into(),
                value: ListComprehension { elements: vec![] }.into(),
                location: Location { row: 4, column: 10 },
            }
            .into(),
            DeclareImmutable {
                variable: UnscopedVariable {
                    name: list3,
                    location: Location { row: 5, column: 14 }
                }
                .into(),
                value: ListComprehension {
                    elements: vec![
                        StringConstant {
                            value: String::from("hello")
                        }
                        .into(),
                        StringConstant {
                            value: String::from("world")
                        }
                        .into(),
                    ],
                }
                .into(),
                location: Location { row: 5, column: 10 },
            }
            .into()
        ]]
    );
}

#[test]
fn can_parse_sets() {
    let mut ctx = Context::new();
    let source = r#"
        (identifier)
        {
          let set1 = {1, 2, 3}
          let set2 = {}
          let set3 = {"hello", "world",}
        }
    "#;
    let mut file = File::new(tree_sitter_python::language());
    file.parse(&mut ctx, source).expect("Cannot parse file");

    let set1 = ctx.add_identifier("set1");
    let set2 = ctx.add_identifier("set2");
    let set3 = ctx.add_identifier("set3");

    let statements = file
        .stanzas
        .into_iter()
        .map(|s| s.statements)
        .collect::<Vec<_>>();
    assert_eq!(
        statements,
        vec![vec![
            DeclareImmutable {
                variable: UnscopedVariable {
                    name: set1,
                    location: Location { row: 3, column: 14 }
                }
                .into(),
                value: SetComprehension {
                    elements: vec![
                        IntegerConstant { value: 1 }.into(),
                        IntegerConstant { value: 2 }.into(),
                        IntegerConstant { value: 3 }.into(),
                    ],
                }
                .into(),
                location: Location { row: 3, column: 10 },
            }
            .into(),
            DeclareImmutable {
                variable: UnscopedVariable {
                    name: set2,
                    location: Location { row: 4, column: 14 }
                }
                .into(),
                value: SetComprehension { elements: vec![] }.into(),
                location: Location { row: 4, column: 10 },
            }
            .into(),
            DeclareImmutable {
                variable: UnscopedVariable {
                    name: set3,
                    location: Location { row: 5, column: 14 }
                }
                .into(),
                value: SetComprehension {
                    elements: vec![
                        StringConstant {
                            value: String::from("hello")
                        }
                        .into(),
                        StringConstant {
                            value: String::from("world")
                        }
                        .into(),
                    ],
                }
                .into(),
                location: Location { row: 5, column: 10 },
            }
            .into()
        ]]
    );
}

#[test]
fn can_parse_print() {
    let mut ctx = Context::new();
    let source = r#"
        (identifier)
        {
          print "x =", 5
        }    
    "#;
    let mut file = File::new(tree_sitter_python::language());
    file.parse(&mut ctx, source).expect("Cannot parse file");

    let statements = file
        .stanzas
        .into_iter()
        .map(|s| s.statements)
        .collect::<Vec<_>>();
    assert_eq!(
        statements,
        vec![vec![Print {
            values: vec![
                StringConstant {
                    value: String::from("x =")
                }
                .into(),
                IntegerConstant { value: 5 }.into(),
            ],
            location: Location { row: 3, column: 10 },
        }
        .into()]]
    );
}

#[test]
fn cannot_parse_nullable_regex() {
    let mut ctx = Context::new();
    let source = r#"
        (module) @root
        {
          scan "abc" {
            "|" {
            }
          }
          node n
        }
    "#;
    let mut file = File::new(tree_sitter_python::language());
    if let Ok(_) = file.parse(&mut ctx, source) {
        panic!("Parse succeeded unexpectedly");
    }
}
