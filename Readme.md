# Rusty Ruby

A modern, high-performance Ruby implementation written in Rust, designed for speed, safety, and reliability.

## 🎯 Project Overview

Rusty Ruby is a comprehensive Ruby implementation that combines the elegance of Ruby with the performance and safety of Rust. It features a multi-tier compilation architecture, advanced garbage collection, and a rich ecosystem of tools and libraries.

## 🌟 Key Features

- **Multi-tier Compilation**: Interpreter, baseline compiler, and optimizing compiler for balanced performance
- **Advanced Garbage Collection**: Generational, incremental, and concurrent GC strategies
- **JIT Compilation**: Just-In-Time compilation for hot code paths
- **WebAssembly Support**: Run Ruby code in the browser with WASI
- **Language Server Protocol**: Modern IDE integration
- **Comprehensive Tooling**: Pure Rust implementations of ruby, irb, gem, rake, rdoc, ri, and testrb
- **Type Safety**: Core type definitions for seamless Rust-Ruby integration
- **Macros**: Procedural macros for easy Ruby class and method definition in Rust

## 🚀 Quick Start

```bash
# Build the project
cargo build

# Run Ruby scripts
cargo run --bin ruby script.rb

# Start interactive Ruby shell
cargo run --bin irb
```

## 📁 Project Structure

The Rusty Ruby project is organized into multiple crates:

- **compilers/ruby**: Main Ruby compiler and runtime
- **compilers/ruby-ir**: Intermediate Representation for Ruby code
- **compilers/ruby-lsp**: Language Server Protocol implementation
- **compilers/ruby-macros**: Procedural macros for Ruby integration
- **compilers/ruby-tools**: Ruby language tools (ruby, irb, gem, etc.)
- **compilers/ruby-types**: Core type definitions
- **compilers/ruby-wasi**: WebAssembly support
- **frontends/ruby-ts**: TypeScript frontend for WASM integration

## 🔧 Development

```bash
# Run tests
cargo test

# Build in release mode
cargo build --release

# Run benchmarks
cargo bench
```

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve Rusty Ruby.

## 📄 Emoji Guide

| Emoji  | Meaning                      |  
|--------|------------------------------|  
| 🎂     | Project initialized!         |  
| 🎉     | Release new version          |  
| 🧪🔮   | Experimental code            |   
| 🔧🐛🐞 | Bug fix                      |  
| 🔒     | Security fix                 |  
| 🐣🐤🐥 | Add feature                  |  
| 📝     | Documentation                |  
| 🚀     | Performance improve!         |  
| 🚧     | Work in progress             |  
| 🚨     | Test coverage improve!       |  
| 🚥     | CI improve!                  |  
| 🔥     | Remove code or files         |
| 🧹     | Code refactor                |
| 📈     | Add analytics or branch code |
| 🤖     | Automation fix               |
| 📦     | Update dependencies          |