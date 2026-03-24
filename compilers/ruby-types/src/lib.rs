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
                if let Some(element) = arr.get_mut(index) {
                    *element = value;
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

    // Array operations

    /// Push element to array
    pub fn push(&mut self, value: RubyValue) -> Option<usize> {
        match self {
            RubyValue::Array(arr) => {
                arr.push(value);
                Some(arr.len())
            }
            _ => None,
        }
    }

    /// Pop element from array
    pub fn pop(&mut self) -> Option<RubyValue> {
        match self {
            RubyValue::Array(arr) => arr.pop(),
            _ => None,
        }
    }

    /// Shift element from array
    pub fn shift(&mut self) -> Option<RubyValue> {
        match self {
            RubyValue::Array(arr) => {
                if arr.is_empty() {
                    None
                }
                else {
                    Some(arr.remove(0))
                }
            }
            _ => None,
        }
    }

    /// Unshift element to array
    pub fn unshift(&mut self, value: RubyValue) -> Option<usize> {
        match self {
            RubyValue::Array(arr) => {
                arr.insert(0, value);
                Some(arr.len())
            }
            _ => None,
        }
    }

    /// Delete element at index
    pub fn delete_at(&mut self, index: usize) -> Option<RubyValue> {
        match self {
            RubyValue::Array(arr) => {
                if index < arr.len() {
                    Some(arr.remove(index))
                }
                else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Insert element at index
    pub fn insert(&mut self, index: usize, value: RubyValue) -> Option<usize> {
        match self {
            RubyValue::Array(arr) => {
                if index <= arr.len() {
                    arr.insert(index, value);
                    Some(arr.len())
                }
                else {
                    None
                }
            }
            _ => None,
        }
    }

    // Hash operations

    /// Delete key from hash
    pub fn delete(&mut self, key: &str) -> Option<RubyValue> {
        match self {
            RubyValue::Hash(map) => map.remove(key),
            _ => None,
        }
    }

    /// Clear hash
    pub fn clear(&mut self) -> Option<()> {
        match self {
            RubyValue::Hash(map) => {
                map.clear();
                Some(())
            }
            _ => None,
        }
    }

    /// Merge hash with another hash
    pub fn merge(&mut self, other: &RubyValue) -> Option<()> {
        match (self, other) {
            (RubyValue::Hash(map1), RubyValue::Hash(map2)) => {
                for (key, value) in map2 {
                    map1.insert(key.clone(), value.clone());
                }
                Some(())
            }
            _ => None,
        }
    }

    /// Select key-value pairs based on predicate
    pub fn select(&self, predicate: impl Fn(&str, &RubyValue) -> bool) -> Option<RubyValue> {
        match self {
            RubyValue::Hash(map) => {
                let mut result = std::collections::HashMap::new();
                for (key, value) in map {
                    if predicate(key, value) {
                        result.insert(key.clone(), value.clone());
                    }
                }
                Some(RubyValue::Hash(result))
            }
            _ => None,
        }
    }

    /// Reject key-value pairs based on predicate
    pub fn reject(&self, predicate: impl Fn(&str, &RubyValue) -> bool) -> Option<RubyValue> {
        match self {
            RubyValue::Hash(map) => {
                let mut result = std::collections::HashMap::new();
                for (key, value) in map {
                    if !predicate(key, value) {
                        result.insert(key.clone(), value.clone());
                    }
                }
                Some(RubyValue::Hash(result))
            }
            _ => None,
        }
    }

    // String operations

    /// Get string length
    pub fn length(&self) -> Option<usize> {
        match self {
            RubyValue::String(s) => Some(s.len()),
            _ => None,
        }
    }

    /// Slice string
    pub fn slice(&self, start: usize, end: Option<usize>) -> Option<RubyValue> {
        match self {
            RubyValue::String(s) => {
                let end_idx = end.unwrap_or(s.len());
                if start <= end_idx && end_idx <= s.len() { Some(RubyValue::String(s[start..end_idx].to_string())) } else { None }
            }
            _ => None,
        }
    }

    /// Split string
    pub fn split(&self, separator: &str) -> Option<RubyValue> {
        match self {
            RubyValue::String(s) => {
                let parts: Vec<RubyValue> = s.split(separator).map(|part| RubyValue::String(part.to_string())).collect();
                Some(RubyValue::Array(parts))
            }
            _ => None,
        }
    }

    /// Convert string to uppercase
    pub fn upcase(&self) -> Option<RubyValue> {
        match self {
            RubyValue::String(s) => Some(RubyValue::String(s.to_uppercase())),
            _ => None,
        }
    }

    /// Convert string to lowercase
    pub fn downcase(&self) -> Option<RubyValue> {
        match self {
            RubyValue::String(s) => Some(RubyValue::String(s.to_lowercase())),
            _ => None,
        }
    }

    // Numeric operations

    /// Get absolute value
    pub fn abs(&self) -> Option<RubyValue> {
        match self {
            RubyValue::Integer(i) => Some(RubyValue::Integer(i.abs())),
            RubyValue::Float(f) => Some(RubyValue::Float(f.abs())),
            _ => None,
        }
    }

    /// Round numeric value
    pub fn round(&self) -> Option<RubyValue> {
        match self {
            RubyValue::Integer(i) => Some(RubyValue::Integer(*i)),
            RubyValue::Float(f) => Some(RubyValue::Float(f.round())),
            _ => None,
        }
    }

    /// Floor numeric value
    pub fn floor(&self) -> Option<RubyValue> {
        match self {
            RubyValue::Integer(i) => Some(RubyValue::Integer(*i)),
            RubyValue::Float(f) => Some(RubyValue::Float(f.floor())),
            _ => None,
        }
    }

    /// Ceil numeric value
    pub fn ceil(&self) -> Option<RubyValue> {
        match self {
            RubyValue::Integer(i) => Some(RubyValue::Integer(*i)),
            RubyValue::Float(f) => Some(RubyValue::Float(f.ceil())),
            _ => None,
        }
    }

    /// Square root of numeric value
    pub fn sqrt(&self) -> Option<RubyValue> {
        match self {
            RubyValue::Integer(i) => {
                if *i >= 0 {
                    Some(RubyValue::Float((*i as f64).sqrt()))
                }
                else {
                    None
                }
            }
            RubyValue::Float(f) => {
                if *f >= 0.0 {
                    Some(RubyValue::Float(f.sqrt()))
                }
                else {
                    None
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_operations() {
        // Test push
        let mut arr = RubyValue::Array(vec![RubyValue::Integer(1), RubyValue::Integer(2)]);
        assert_eq!(arr.push(RubyValue::Integer(3)), Some(3));
        assert_eq!(arr.array_length(), Some(3));

        // Test pop
        assert_eq!(arr.pop(), Some(RubyValue::Integer(3)));
        assert_eq!(arr.array_length(), Some(2));

        // Test shift
        assert_eq!(arr.shift(), Some(RubyValue::Integer(1)));
        assert_eq!(arr.array_length(), Some(1));

        // Test unshift
        assert_eq!(arr.unshift(RubyValue::Integer(0)), Some(2));
        assert_eq!(arr.array_length(), Some(2));

        // Test delete_at
        assert_eq!(arr.delete_at(1), Some(RubyValue::Integer(2)));
        assert_eq!(arr.array_length(), Some(1));

        // Test insert
        assert_eq!(arr.insert(1, RubyValue::Integer(1)), Some(2));
        assert_eq!(arr.array_length(), Some(2));
    }

    #[test]
    fn test_hash_operations() {
        // Test delete
        let mut hash = RubyValue::Hash(std::collections::HashMap::from([
            ("a".to_string(), RubyValue::Integer(1)),
            ("b".to_string(), RubyValue::Integer(2)),
        ]));
        assert_eq!(hash.delete("a"), Some(RubyValue::Integer(1)));
        assert_eq!(hash.hash_get("a"), None);

        // Test clear
        assert_eq!(hash.clear(), Some(()));
        assert_eq!(hash.hash_keys(), Some(vec![]));

        // Test merge
        let mut hash1 = RubyValue::Hash(std::collections::HashMap::from([("a".to_string(), RubyValue::Integer(1))]));
        let hash2 = RubyValue::Hash(std::collections::HashMap::from([("b".to_string(), RubyValue::Integer(2))]));
        assert_eq!(hash1.merge(&hash2), Some(()));
        assert_eq!(hash1.hash_get("b"), Some(RubyValue::Integer(2)));

        // Test select
        let hash = RubyValue::Hash(std::collections::HashMap::from([
            ("a".to_string(), RubyValue::Integer(1)),
            ("b".to_string(), RubyValue::Integer(2)),
            ("c".to_string(), RubyValue::Integer(3)),
        ]));
        let result = hash.select(|_, v| v.to_i32() > 1);
        if let Some(RubyValue::Hash(map)) = result {
            assert_eq!(map.len(), 2);
            assert_eq!(map.get("b"), Some(&RubyValue::Integer(2)));
            assert_eq!(map.get("c"), Some(&RubyValue::Integer(3)));
        }
        else {
            panic!("select should return a hash");
        }

        // Test reject
        let result = hash.reject(|_, v| v.to_i32() > 1);
        if let Some(RubyValue::Hash(map)) = result {
            assert_eq!(map.len(), 1);
            assert_eq!(map.get("a"), Some(&RubyValue::Integer(1)));
        }
        else {
            panic!("reject should return a hash");
        }
    }

    #[test]
    fn test_string_operations() {
        let s = RubyValue::String("Hello World".to_string());

        // Test length
        assert_eq!(s.length(), Some(11));

        // Test slice
        assert_eq!(s.slice(0, Some(5)), Some(RubyValue::String("Hello".to_string())));
        assert_eq!(s.slice(6, None), Some(RubyValue::String("World".to_string())));

        // Test split
        let result = s.split(" ");
        if let Some(RubyValue::Array(arr)) = result {
            assert_eq!(arr.len(), 2);
            assert_eq!(arr[0], RubyValue::String("Hello".to_string()));
            assert_eq!(arr[1], RubyValue::String("World".to_string()));
        }
        else {
            panic!("split should return an array");
        }

        // Test upcase
        assert_eq!(s.upcase(), Some(RubyValue::String("HELLO WORLD".to_string())));

        // Test downcase
        assert_eq!(s.downcase(), Some(RubyValue::String("hello world".to_string())));
    }

    #[test]
    fn test_numeric_operations() {
        // Test abs
        let neg_int = RubyValue::Integer(-5);
        assert_eq!(neg_int.abs(), Some(RubyValue::Integer(5)));
        let neg_float = RubyValue::Float(-3.14);
        assert_eq!(neg_float.abs(), Some(RubyValue::Float(3.14)));

        // Test round
        let float = RubyValue::Float(3.6);
        assert_eq!(float.round(), Some(RubyValue::Float(4.0)));

        // Test floor
        assert_eq!(float.floor(), Some(RubyValue::Float(3.0)));

        // Test ceil
        assert_eq!(float.ceil(), Some(RubyValue::Float(4.0)));

        // Test sqrt
        let int = RubyValue::Integer(4);
        assert_eq!(int.sqrt(), Some(RubyValue::Float(2.0)));
        let float = RubyValue::Float(9.0);
        assert_eq!(float.sqrt(), Some(RubyValue::Float(3.0)));
    }
}
