//! Ruby types for Rusty Ruby
//!
//! This crate provides core Ruby value and error types for the Rusty Ruby ecosystem.

#![warn(missing_docs)]

/// Ruby value type
#[derive(Debug, Clone, PartialEq)]
pub enum RubyValue {
    /// Nil value
    Nil,
    /// Boolean value
    Boolean(bool),
    /// Integer value
    Integer(i32),
    /// Float value
    Float(f64),
    /// String value
    String(String),
    /// Symbol value
    Symbol(String),
    /// Array value
    Array(Vec<RubyValue>),
    /// Hash value
    Hash(std::collections::HashMap<String, RubyValue>),
    /// Object value
    Object(String, std::collections::HashMap<String, RubyValue>),
}

impl RubyValue {
    /// Check if value is nil
    pub fn is_nil(&self) -> bool {
        matches!(self, RubyValue::Nil)
    }

    /// Convert value to i32
    pub fn to_i32(&self) -> i32 {
        match self {
            RubyValue::Integer(i) => *i,
            RubyValue::Float(f) => *f as i32,
            RubyValue::Boolean(b) => {
                if *b {
                    1
                }
                else {
                    0
                }
            }
            _ => 0,
        }
    }

    /// Convert value to f64
    pub fn to_f64(&self) -> f64 {
        match self {
            RubyValue::Float(f) => *f,
            RubyValue::Integer(i) => *i as f64,
            RubyValue::Boolean(b) => {
                if *b {
                    1.0
                }
                else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    /// Convert value to bool
    pub fn to_bool(&self) -> bool {
        match self {
            RubyValue::Nil => false,
            RubyValue::Boolean(b) => *b,
            RubyValue::Integer(i) => *i != 0,
            RubyValue::Float(f) => *f != 0.0,
            _ => true,
        }
    }

    /// Convert value to string
    pub fn to_string(&self) -> String {
        match self {
            RubyValue::String(s) => s.clone(),
            RubyValue::Symbol(s) => s.clone(),
            RubyValue::Integer(i) => i.to_string(),
            RubyValue::Float(f) => f.to_string(),
            RubyValue::Boolean(b) => b.to_string(),
            RubyValue::Nil => "nil".to_string(),
            _ => format!("{:?}", self),
        }
    }
}

/// Ruby error type
#[derive(Debug, Clone, PartialEq)]
pub enum RubyError {
    /// Method not found error
    MethodNotFound(String),
    /// Class not found error
    ClassNotFound(String),
    /// Lexical analysis error
    LexicalError(String),
    /// Syntax analysis error
    SyntaxError(String),
    /// Runtime error
    RuntimeError(String),
    /// Type error
    TypeError(String),
    /// Argument error
    ArgumentError(String),
}

impl std::fmt::Display for RubyError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RubyError::MethodNotFound(name) => write!(f, "Method not found: {}", name),
            RubyError::ClassNotFound(name) => write!(f, "Class not found: {}", name),
            RubyError::LexicalError(msg) => write!(f, "Lexical error: {}", msg),
            RubyError::SyntaxError(msg) => write!(f, "Syntax error: {}", msg),
            RubyError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            RubyError::TypeError(msg) => write!(f, "Type error: {}", msg),
            RubyError::ArgumentError(msg) => write!(f, "Argument error: {}", msg),
        }
    }
}

impl std::error::Error for RubyError {}

/// Ruby result type
pub type RubyResult<T> = std::result::Result<T, RubyError>;
