# Ruby Macros

This crate provides procedural macros for Rusty Ruby, including `derive(RubyClass)` and related macros to simplify the process of defining Ruby classes and methods in Rust.

## Features

- `derive(RubyClass)`: Automatically generates Ruby class definitions from Rust structs
- `#[ruby_method]`: Attribute macro for defining Ruby methods
- Seamless integration with the Rusty Ruby runtime

## Usage

### Define a Ruby class

```rust
use ruby_macros::RubyClass;

#[derive(RubyClass)]
pub struct MyClass {
    value: i32,
}

impl MyClass {
    #[ruby_method]
    pub fn new(&self, value: i32) -> Self {
        Self { value }
    }
    
    #[ruby_method]
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
```

## Dependencies

- `syn` - For parsing Rust code
- `quote` - For generating Rust code
- `proc-macro2` - For working with procedural macros