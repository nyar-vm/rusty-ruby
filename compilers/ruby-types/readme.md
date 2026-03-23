# Ruby Types

This crate provides the core type definitions for the Rusty Ruby ecosystem, including RubyValue, RubyError, and related types.

## Features

- `RubyValue`: A unified type for representing Ruby values in Rust
- `RubyError`: Error types for Ruby operations
- `RubyResult`: Result type for Ruby operations
- Traits for converting between Rust and Ruby types

## Usage

```rust
use ruby_types::{RubyValue, RubyError, RubyResult, ToRubyValue, FromRubyValue};

// Create a Ruby value
let ruby_string = RubyValue::String("Hello, Ruby!".to_string());

// Convert from Rust types
let rust_string: String = "Hello, Rust!".to_string();
let ruby_string = rust_string.to_ruby_value();

// Convert to Rust types
let ruby_int = RubyValue::Integer(42);
let rust_int: i32 = ruby_int.into_rust_value().unwrap();
```

## Dependencies

- This crate has no external dependencies, making it lightweight and easy to integrate