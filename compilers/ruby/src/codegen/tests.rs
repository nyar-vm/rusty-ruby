//! Tests for code generation

use super::*;
use oak_ruby::ast::{ExpressionNode, LiteralNode, RubyAst, StatementNode};
use ruby_ir::{Expression, Statement};

#[test]
fn test_ast_to_ir_basic() {
    // Create a simple AST with a single expression
    let ast = RubyAst {
        statements: vec![
            StatementNode::Expression(Box::new(ExpressionNode::Literal(LiteralNode::Integer { value: 42, span: Default::default() })))
        ],
        span: Default::default(),
    };

    // Convert to IR
    let program = ast_to_ir(&ast).unwrap();

    // Verify the result
    assert_eq!(program.global_statements.len(), 1);
    match &program.global_statements[0] {
        Statement::Expression(expr) => {
            match expr {
                Expression::Literal(RubyValue::Integer(42)) => {},
                _ => panic!("Expected literal 42, got {:?}", expr),
            }
        },
        _ => panic!("Expected expression statement, got {:?}", program.global_statements[0]),
    }
}

#[test]
fn test_ast_to_ir_assignment() {
    // Create an AST with an assignment
    let ast = RubyAst {
        statements: vec![
            StatementNode::Assignment {
                target: "x".to_string(),
                value: Box::new(ExpressionNode::Literal(LiteralNode::Integer { value: 10, span: Default::default() })),
                span: Default::default(),
            }
        ],
        span: Default::default(),
    };

    // Convert to IR
    let program = ast_to_ir(&ast).unwrap();

    // Verify the result
    assert_eq!(program.global_statements.len(), 1);
    match &program.global_statements[0] {
        Statement::Assignment { name, value } => {
            assert_eq!(name, "x");
            match value {
                Expression::Literal(RubyValue::Integer(10)) => {},
                _ => panic!("Expected literal 10, got {:?}", value),
            }
        },
        _ => panic!("Expected assignment statement, got {:?}", program.global_statements[0]),
    }
}

#[test]
fn test_ast_to_ir_method_def() {
    // Create an AST with a method definition
    let ast = RubyAst {
        statements: vec![
            StatementNode::MethodDef {
                name: "test".to_string(),
                params: vec!["a".to_string(), "b".to_string()],
                body: vec![
                    StatementNode::Return {
                        value: Some(Box::new(ExpressionNode::Identifier { name: "a".to_string(), span: Default::default() })),
                        span: Default::default(),
                    }
                ],
                span: Default::default(),
            }
        ],
        span: Default::default(),
    };

    // Convert to IR
    let program = ast_to_ir(&ast).unwrap();

    // Verify the result
    assert_eq!(program.global_statements.len(), 1);
    match &program.global_statements[0] {
        Statement::MethodDefinition { name, parameters, body } => {
            assert_eq!(name, "test");
            assert_eq!(parameters, &vec!["a", "b"]);
            assert_eq!(body.len(), 1);
        },
        _ => panic!("Expected method definition statement, got {:?}", program.global_statements[0]),
    }
    assert_eq!(program.functions.len(), 1);
}

#[test]
fn test_ast_to_ir_class_def() {
    // Create an AST with a class definition
    let ast = RubyAst {
        statements: vec![
            StatementNode::ClassDef {
                name: "Test".to_string(),
                superclass: Some("Object".to_string()),
                body: vec![
                    StatementNode::MethodDef {
                        name: "initialize".to_string(),
                        params: vec!["value".to_string()],
                        body: vec![
                            StatementNode::Assignment {
                                target: "@value".to_string(),
                                value: Box::new(ExpressionNode::Identifier { name: "value".to_string(), span: Default::default() }),
                                span: Default::default(),
                            }
                        ],
                        span: Default::default(),
                    }
                ],
                span: Default::default(),
            }
        ],
        span: Default::default(),
    };

    // Convert to IR
    let program = ast_to_ir(&ast).unwrap();

    // Verify the result
    assert_eq!(program.global_statements.len(), 1);
    match &program.global_statements[0] {
        Statement::ClassDefinition { name, superclass, body } => {
            assert_eq!(name, "Test");
            assert_eq!(superclass, &Some("Object".to_string()));
            assert_eq!(body.len(), 1);
        },
        _ => panic!("Expected class definition statement, got {:?}", program.global_statements[0]),
    }
    assert_eq!(program.classes.len(), 1);
    assert_eq!(program.functions.len(), 1);
}

#[test]
fn test_ast_to_ir_self_reference() {
    // Create an AST with a self reference
    let ast = RubyAst {
        statements: vec![
            StatementNode::Expression(Box::new(ExpressionNode::Identifier { name: "self".to_string(), span: Default::default() }))
        ],
        span: Default::default(),
    };

    // Convert to IR
    let program = ast_to_ir(&ast).unwrap();

    // Verify the result
    assert_eq!(program.global_statements.len(), 1);
    match &program.global_statements[0] {
        Statement::Expression(expr) => {
            match expr {
                Expression::SelfRef => {},
                _ => panic!("Expected self reference, got {:?}", expr),
            }
        },
        _ => panic!("Expected expression statement, got {:?}", program.global_statements[0]),
    }
}
