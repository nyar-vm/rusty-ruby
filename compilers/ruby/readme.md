# 🎯 Rusty Ruby Runtime

A **pure Rust** implementation of Ruby runtime environment.

## Overview

`rusty-ruby` is a high-performance Ruby runtime environment built entirely in Rust. It brings the elegance and programmer-centric philosophy of Ruby to a modern, optimized runtime, enabling Ruby applications to benefit from Rust's safety and performance.

## ✨ Features

- **Pure Rust Implementation**: 🦀 Built entirely in Rust for maximum safety and performance
- **Runtime Environment**: 🖥️ Basic Ruby runtime environment with value types and execution framework
- **Expressive Syntax**: 💬 Supports Ruby's expressive syntax and dynamic object model
- **Lightweight Design**: 📦 Minimal dependencies, focused on core functionality

## 🛠️ Supported Features

- **Core Syntax**: `class`, `module`, `def`, `attr_accessor`
- **Control Flow**: `if`, `unless`, `while`, `until`
- **Value Types**: `nil`, `boolean`, `integer`, `float`, `string`, `symbol`, `array`, `hash`
- **Basic Execution**: Script execution framework

## 🚀 Getting Started

### Usage as a Library

```rust
use ruby::Ruby;

// Create a new Ruby runtime
let mut ruby = Ruby::new().unwrap();

// Execute a Ruby script
ruby.execute_script("puts 'Hello, Rusty Ruby!'")
    .expect("Script execution failed");

// Define a class
ruby.define_class("Person").unwrap();

// Define a method
ruby.define_method("Person", "greet", Box::new(|| {
    println!("Hello from Person class!");
})).unwrap();
```

## 📦 Dependencies

- **oak-core**: Rust-based parsing framework
- **oak-ruby**: Ruby language frontend
- **oak-vfs**: Virtual file system support

## 📄 License

Licensed under MIT OR Apache-2.0.

## 🌟 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve this project.
