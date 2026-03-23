# Ruby Types

Core type definitions for the Rusty Ruby ecosystem, providing a foundation for Ruby-Rust interop.

## 🎯 Project Overview

Ruby-Types is a crate that provides the core type definitions for the Rusty Ruby ecosystem, including `RubyValue`, `RubyError`, and related types. It serves as the foundation for type conversion between Rust and Ruby.

## 🌟 Key Features

- **`RubyValue`**: A unified type for representing Ruby values in Rust
- **`RubyError`**: Error types for Ruby operations
- **`RubyResult`**: Result type for Ruby operations
- **Type Conversion Traits**: Traits for converting between Rust and Ruby types
- **Lightweight**: No external dependencies, making it easy to integrate
- **Comprehensive**: Covers all Ruby value types

## 🚀 Quick Start

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

// Handle Ruby errors
fn safe_operation() -> RubyResult<i32> {
    // Perform operation that might fail
    Ok(42)
}

match safe_operation() {
    Ok(value) => println!("Success: {}", value),
    Err(error) => println!("Error: {:?}", error),
}
```

## 🏗️ Architecture

### Core Types

- **`RubyValue`**: Represents all possible Ruby values, including integers, floats, strings, arrays, hashes, symbols, booleans, nil, and objects
- **`RubyError`**: Represents errors that can occur during Ruby operations
- **`RubyResult`**: A convenience type alias for `Result<T, RubyError>`

### Type Conversion Traits

- **`ToRubyValue`**: Converts Rust types to Ruby values
- **`FromRubyValue`**: Converts Ruby values to Rust types
- **`RubyObject`**: Trait for Rust types that can be wrapped as Ruby objects

## 🛠️ Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Build in release mode
cargo build --release
```

## 📚 Usage Examples

### Basic Type Conversion

```rust
use ruby_types::{RubyValue, ToRubyValue, FromRubyValue};

// Convert Rust types to Ruby values
let ruby_int = 42.to_ruby_value();
let ruby_float = 3.14.to_ruby_value();
let ruby_string = "Hello".to_ruby_value();
let ruby_bool = true.to_ruby_value();
let ruby_nil = ().to_ruby_value(); // Represents Ruby nil

// Convert Ruby values to Rust types
let rust_int: i32 = ruby_int.into_rust_value().unwrap();
let rust_float: f64 = ruby_float.into_rust_value().unwrap();
let rust_string: String = ruby_string.into_rust_value().unwrap();
let rust_bool: bool = ruby_bool.into_rust_value().unwrap();

// Handle collections
let rust_array = vec![1, 2, 3];
let ruby_array = rust_array.to_ruby_value();

let rust_hash = vec![("key1", "value1"), ("key2", "value2")]
    .into_iter()
    .collect::<std::collections::HashMap<_, _>>();
let ruby_hash = rust_hash.to_ruby_value();
```

### Custom Ruby Objects

```rust
use ruby_types::{RubyValue, RubyObject};

#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
}

impl RubyObject for Person {
    fn class_name(&self) -> &str {
        "Person"
    }
    
    fn to_ruby_value(&self) -> RubyValue {
        // Convert Person to a Ruby object
        RubyValue::Object(Box::new(self.clone()))
    }
}

// Create a Person and convert to Ruby value
let person = Person { name: "Alice".to_string(), age: 30 };
let ruby_person = person.to_ruby_value();
```

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve Ruby-Types.
