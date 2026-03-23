# Ruby Macros

Procedural macros for Rusty Ruby, simplifying the process of defining Ruby classes and methods in Rust.

## 🎯 Project Overview

Ruby-Macros is a crate that provides procedural macros for Rusty Ruby, including `derive(RubyClass)` and related macros to simplify the process of defining Ruby classes and methods in Rust.

## 🌟 Key Features

- **`derive(RubyClass)`**: Automatically generates Ruby class definitions from Rust structs
- **`#[ruby_method]`**: Attribute macro for defining Ruby methods
- **Seamless Integration**: Works smoothly with the Rusty Ruby runtime
- **Type Safety**: Maintains Rust's type safety while providing Ruby interop
- **Minimal Boilerplate**: Reduces repetitive code for Ruby class definitions

## 🚀 Quick Start

### Define a Ruby Class

```rust
use ruby_macros::RubyClass;

#[derive(RubyClass)]
pub struct MyClass {
    value: i32,
}

impl MyClass {
    #[ruby_method]
    pub fn new(value: i32) -> Self {
        Self { value }
    }
    
    #[ruby_method]
    pub fn get_value(&self) -> i32 {
        self.value
    }
    
    #[ruby_method]
    pub fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}
```

### Use the Ruby Class

```rust
use ruby::Ruby;
use crate::MyClass;

fn main() -> ruby::Result<()> {
    let mut ruby = Ruby::new()?;
    
    // The RubyClass derive macro automatically registers the class
    // with the Ruby runtime
    
    // Create an instance of MyClass from Ruby
    ruby.execute_script("obj = MyClass.new(42)")?;
    
    // Call methods on the instance
    ruby.execute_script("puts obj.get_value")?; // Output: 42
    ruby.execute_script("obj.set_value(100)")?;
    ruby.execute_script("puts obj.get_value")?; // Output: 100
    
    Ok(())
}
```

## 🏗️ Architecture

### Macro Expansion

The `derive(RubyClass)` macro expands to:
1. A Ruby class definition
2. Methods for converting between Rust and Ruby types
3. Registration code for the Ruby runtime
4. Implementation of required traits

The `#[ruby_method]` macro expands to:
1. A Ruby method definition
2. Type conversion code for arguments and return values
3. Error handling for Ruby method calls

## 🛠️ Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Build in release mode
cargo build --release
```

## 📚 Advanced Usage

### Class Inheritance

```rust
use ruby_macros::RubyClass;

#[derive(RubyClass)]
pub struct BaseClass {
    base_value: i32,
}

impl BaseClass {
    #[ruby_method]
    pub fn new(base_value: i32) -> Self {
        Self { base_value }
    }
    
    #[ruby_method]
    pub fn base_method(&self) -> i32 {
        self.base_value
    }
}

#[derive(RubyClass)]
#[ruby_class(superclass = "BaseClass")]
pub struct SubClass {
    #[ruby_class(skip)] // Skip this field from Ruby access
    rust_only_field: String,
    sub_value: i32,
}

impl SubClass {
    #[ruby_method]
    pub fn new(base_value: i32, sub_value: i32) -> Self {
        Self {
            base_value,
            rust_only_field: "rust-only".to_string(),
            sub_value,
        }
    }
    
    #[ruby_method]
    pub fn sub_method(&self) -> i32 {
        self.sub_value
    }
}
```

### Method Overloading

```rust
use ruby_macros::RubyClass;

#[derive(RubyClass)]
pub struct Calculator {
    value: i32,
}

impl Calculator {
    #[ruby_method]
    pub fn new() -> Self {
        Self { value: 0 }
    }
    
    #[ruby_method(name = "add")]
    pub fn add_i32(&mut self, value: i32) {
        self.value += value;
    }
    
    #[ruby_method(name = "add")]
    pub fn add_f64(&mut self, value: f64) {
        self.value += value as i32;
    }
    
    #[ruby_method]
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
```

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve Ruby-Macros.
