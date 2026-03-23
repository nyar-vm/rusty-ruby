//! Ruby Intermediate Representation (IR)
//!
//! This crate provides a structured intermediate representation for Ruby code,
//! designed to facilitate optimization passes and code generation.

#![warn(missing_docs)]

use ruby_types::{RubyError, RubyResult, RubyValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type alias for Ruby result
pub type Result<T> = RubyResult<T>;

/// IR node ID type
pub type NodeId = u32;

/// IR basic block ID type
pub type BlockId = u32;

/// IR function ID type
pub type FunctionId = u32;

/// IR class ID type
pub type ClassId = u32;

/// IR module ID type
pub type ModuleId = u32;

/// Expression types in the IR
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    /// Literal value
    Literal(RubyValue),
    /// Variable reference
    Variable(String),
    /// Global variable reference
    GlobalVariable(String),
    /// Instance variable reference
    InstanceVariable(String),
    /// Class variable reference
    ClassVariable(String),
    /// Method call
    MethodCall {
        /// Method receiver
        receiver: Box<Expression>,
        /// Method name
        method: String,
        /// Method arguments
        arguments: Vec<Expression>,
    },
    /// Binary operation
    BinaryOp {
        /// Left operand
        left: Box<Expression>,
        /// Binary operator
        op: BinaryOperator,
        /// Right operand
        right: Box<Expression>,
    },
    /// Unary operation
    UnaryOp {
        /// Unary operator
        op: UnaryOperator,
        /// Operand
        operand: Box<Expression>,
    },
    /// Array literal
    ArrayLiteral(Vec<Expression>),
    /// Hash literal
    HashLiteral(HashMap<String, Expression>),
    /// Block creation
    Block {
        /// Block parameters
        parameters: Vec<String>,
        /// Block body statements
        body: Vec<Statement>,
    },
    /// Self reference
    SelfRef,
    /// Super call
    SuperCall {
        /// Super call arguments
        arguments: Vec<Expression>,
    },
}

/// Binary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BinaryOperator {
    /// Addition
    Add,
    /// Subtraction
    Sub,
    /// Multiplication
    Mul,
    /// Division
    Div,
    /// Modulo
    Mod,
    /// Exponentiation
    Exp,
    /// Equality
    Eq,
    /// Inequality
    Neq,
    /// Less than
    Lt,
    /// Less than or equal
    Lte,
    /// Greater than
    Gt,
    /// Greater than or equal
    Gte,
    /// Logical AND
    And,
    /// Logical OR
    Or,
    /// Assignment
    Assign,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnaryOperator {
    /// Logical NOT
    Not,
    /// Negation
    Neg,
    /// Plus
    Plus,
    /// Bitwise NOT
    BitNot,
}

/// Statement types in the IR
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    /// Expression statement
    Expression(Expression),
    /// Variable assignment
    Assignment {
        /// Variable name
        name: String,
        /// Assignment value
        value: Expression,
    },
    /// Global variable assignment
    GlobalAssignment {
        /// Global variable name
        name: String,
        /// Assignment value
        value: Expression,
    },
    /// Instance variable assignment
    InstanceAssignment {
        /// Instance variable name
        name: String,
        /// Assignment value
        value: Expression,
    },
    /// Class variable assignment
    ClassAssignment {
        /// Class variable name
        name: String,
        /// Assignment value
        value: Expression,
    },
    /// If statement
    If {
        /// Condition expression
        condition: Expression,
        /// Then branch statements
        then_branch: Vec<Statement>,
        /// Else branch statements
        else_branch: Vec<Statement>,
    },
    /// While loop
    While {
        /// Loop condition
        condition: Expression,
        /// Loop body statements
        body: Vec<Statement>,
    },
    /// Until loop
    Until {
        /// Loop condition
        condition: Expression,
        /// Loop body statements
        body: Vec<Statement>,
    },
    /// Case statement
    Case {
        /// Value to match
        value: Expression,
        /// When clauses (condition, body)
        when_clauses: Vec<(Expression, Vec<Statement>)>,
        /// Else clause statements
        else_clause: Vec<Statement>,
    },
    /// For loop
    For {
        /// Loop variable
        variable: String,
        /// Iterator expression
        iterator: Expression,
        /// Loop body statements
        body: Vec<Statement>,
    },
    /// Break statement
    Break,
    /// Next statement
    Next,
    /// Redo statement
    Redo,
    /// Return statement
    Return(Option<Expression>),
    /// Method definition
    MethodDefinition {
        /// Method name
        name: String,
        /// Method parameters
        parameters: Vec<String>,
        /// Method body statements
        body: Vec<Statement>,
    },
    /// Class definition
    ClassDefinition {
        /// Class name
        name: String,
        /// Superclass name
        superclass: Option<String>,
        /// Class body statements
        body: Vec<Statement>,
    },
    /// Module definition
    ModuleDefinition {
        /// Module name
        name: String,
        /// Module body statements
        body: Vec<Statement>,
    },
    /// Require statement
    Require(Expression),
    /// Load statement
    Load(Expression),
}

/// Basic block in the IR
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasicBlock {
    /// Block ID
    pub id: BlockId,
    /// Statements in the block
    pub statements: Vec<Statement>,
    /// Successor blocks
    pub successors: Vec<BlockId>,
}

/// Function in the IR
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    /// Function ID
    pub id: FunctionId,
    /// Function name
    pub name: String,
    /// Parameters
    pub parameters: Vec<String>,
    /// Basic blocks
    pub blocks: HashMap<BlockId, BasicBlock>,
    /// Entry block ID
    pub entry_block: BlockId,
}

/// Class in the IR
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Class {
    /// Class ID
    pub id: ClassId,
    /// Class name
    pub name: String,
    /// Superclass name
    pub superclass: Option<String>,
    /// Methods
    pub methods: HashMap<String, FunctionId>,
}

/// Module in the IR
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    /// Module ID
    pub id: ModuleId,
    /// Module name
    pub name: String,
    /// Methods
    pub methods: HashMap<String, FunctionId>,
}

/// Program in the IR
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    /// Functions
    pub functions: HashMap<FunctionId, Function>,
    /// Classes
    pub classes: HashMap<ClassId, Class>,
    /// Modules
    pub modules: HashMap<ModuleId, Module>,
    /// Global statements
    pub global_statements: Vec<Statement>,
}

impl Program {
    /// Create a new empty program
    pub fn new() -> Self {
        Self { functions: HashMap::new(), classes: HashMap::new(), modules: HashMap::new(), global_statements: Vec::new() }
    }

    /// Add a function to the program
    pub fn add_function(&mut self, function: Function) {
        self.functions.insert(function.id, function);
    }

    /// Add a class to the program
    pub fn add_class(&mut self, class: Class) {
        self.classes.insert(class.id, class);
    }

    /// Add a module to the program
    pub fn add_module(&mut self, module: Module) {
        self.modules.insert(module.id, module);
    }

    /// Add a global statement
    pub fn add_global_statement(&mut self, statement: Statement) {
        self.global_statements.push(statement);
    }
}

/// Serialize the program to JSON
pub fn serialize_program(program: &Program) -> Result<String> {
    serde_json::to_string(program).map_err(|e| RubyError::RuntimeError(format!("Serialization error: {}", e)))
}

/// Deserialize the program from JSON
pub fn deserialize_program(json: &str) -> Result<Program> {
    serde_json::from_str(json).map_err(|e| RubyError::RuntimeError(format!("Deserialization error: {}", e)))
}

/// Traversal and manipulation utilities
pub mod traversal;

/// Optimization utilities
pub mod optimization;
