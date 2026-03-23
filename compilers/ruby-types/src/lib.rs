//! Ruby types for Rusty Ruby
//!
//! This crate provides core Ruby value and error types for the Rusty Ruby ecosystem.

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};

/// Ruby value type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// Closure value
    Closure(usize), // 使用 usize 作为闭包的唯一标识符
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

    /// Mark this value and all referenced values
    pub fn mark(&self, marked: &mut std::collections::HashSet<*const RubyValue>) {
        let ptr = self as *const RubyValue;
        if marked.contains(&ptr) {
            return;
        }
        marked.insert(ptr);

        match self {
            RubyValue::Array(values) => {
                for value in values {
                    value.mark(marked);
                }
            }
            RubyValue::Hash(map) => {
                for (_, value) in map {
                    value.mark(marked);
                }
            }
            RubyValue::Object(_, fields) => {
                for (_, value) in fields {
                    value.mark(marked);
                }
            }
            _ => {}
        }
    }

    /// Concatenate two Ruby values
    pub fn concat(&self, other: &RubyValue) -> RubyValue {
        match (self, other) {
            (RubyValue::String(s1), RubyValue::String(s2)) => RubyValue::String(format!("{}{}", s1, s2)),
            (RubyValue::Array(arr1), RubyValue::Array(arr2)) => {
                let mut result = arr1.clone();
                result.extend(arr2.clone());
                RubyValue::Array(result)
            }
            _ => RubyValue::String(format!("{}{}", self.to_string(), other.to_string())),
        }
    }

    /// Get array length
    pub fn array_length(&self) -> Option<usize> {
        match self {
            RubyValue::Array(arr) => Some(arr.len()),
            _ => None,
        }
    }

    /// Get array element at index
    pub fn array_get(&self, index: usize) -> Option<RubyValue> {
        match self {
            RubyValue::Array(arr) => arr.get(index).cloned(),
            _ => None,
        }
    }

    /// Set array element at index
    pub fn array_set(&mut self, index: usize, value: RubyValue) -> Option<()> {
        match self {
            RubyValue::Array(arr) => {
                if index < arr.len() {
                    arr[index] = value;
                    Some(())
                }
                else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Get hash value for key
    pub fn hash_get(&self, key: &str) -> Option<RubyValue> {
        match self {
            RubyValue::Hash(map) => map.get(key).cloned(),
            _ => None,
        }
    }

    /// Set hash value for key
    pub fn hash_set(&mut self, key: &str, value: RubyValue) -> Option<()> {
        match self {
            RubyValue::Hash(map) => {
                map.insert(key.to_string(), value);
                Some(())
            }
            _ => None,
        }
    }

    /// Get hash keys
    pub fn hash_keys(&self) -> Option<Vec<String>> {
        match self {
            RubyValue::Hash(map) => Some(map.keys().cloned().collect()),
            _ => None,
        }
    }

    /// Get hash values
    pub fn hash_values(&self) -> Option<Vec<RubyValue>> {
        match self {
            RubyValue::Hash(map) => Some(map.values().cloned().collect()),
            _ => None,
        }
    }

    /// Get object field
    pub fn object_get(&self, field: &str) -> Option<RubyValue> {
        match self {
            RubyValue::Object(_, fields) => fields.get(field).cloned(),
            _ => None,
        }
    }

    /// Set object field
    pub fn object_set(&mut self, field: &str, value: RubyValue) -> Option<()> {
        match self {
            RubyValue::Object(_, fields) => {
                fields.insert(field.to_string(), value);
                Some(())
            }
            _ => None,
        }
    }
}

/// Ruby error type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
