# Rusty Ruby Compiler

A modern, high-performance Ruby compiler and runtime written in Rust, featuring advanced compilation techniques and memory management.

## 🎯 Project Overview

Rusty Ruby is a complete Ruby implementation that combines the elegance of Ruby with the performance and safety of Rust. It features a multi-tier compilation architecture, advanced garbage collection, and a rich ecosystem of tools and libraries.

## 🌟 Key Features

- **Multi-tier Compilation**: Interpreter, baseline compiler, and optimizing compiler for balanced performance
- **Advanced Garbage Collection**: Generational, incremental, and concurrent GC strategies
- **JIT Compilation**: Just-In-Time compilation for hot code paths
- **FFI Interface**: Seamless integration with C libraries
- **Performance Analysis**: Built-in profiler and debugging tools
- **Register-based VM**: Efficient virtual machine implementation

## 🚀 Quick Start

```rust
use ruby::Ruby;

fn main() -> ruby::Result<()> {
    let mut ruby = Ruby::new()?;
    
    // Execute Ruby script
    ruby.execute_script("$result = 1 + 2 * 3")?;
    
    // Get result
    let result = ruby.get_global("$result")?;
    println!("Result: {:?}", result);
    
    Ok(())
}
```

## 🏗️ Architecture

### Compiler Pipeline

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Source Code│ -> │  Lexical    │ -> │  Syntax     │ -> │    AST      │
│             │    │  Analysis   │    │  Analysis   │    │             │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                                              │
                                              ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Execution  │ <- │ Execution   │ <- │  VM         │ <- │    IR       │
│  Results    │    │ Engine      │    │  Instructions│    │             │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                      │
                      ▼
              ┌─────────────────────────────────┐
              │         Multi-tier Compilation  │
              ├─────────────┬─────────────┬──────┘
              │             │             │
       ┌──────▼──────┐ ┌────▼──────┐ ┌────▼──────┐
       │  Interpreter│ │ Baseline  │ │ Optimizing │
       │             │ │ Compiler  │ │ Compiler  │
       └─────────────┘ └────────────┘ └────────────┘
                      │
                      ▼
              ┌─────────────────────────────────┐
              │         Garbage Collection      │
              ├─────────────┬─────────────┬──────┘
              │             │             │
       ┌──────▼──────┐ ┌────▼──────┐ ┌────▼──────┐
       │ Generational│ │ Incremental│ │ Concurrent│
       │ GC          │ │ GC         │ │ GC        │
       └─────────────┘ └────────────┘ └────────────┘
```

## 📁 Directory Structure

```
compilers/ruby/
├── src/
│   ├── lib.rs          # Main entry point, Ruby runtime environment
│   ├── vm.rs           # Register-based virtual machine implementation
│   ├── jit.rs          # JIT compiler implementation
│   ├── codegen.rs      # Code generation (AST to IR to VM instructions)
│   ├── architecture/   # Multi-tier compilation architecture
│   │   ├── mod.rs      # Architecture module definition
│   │   ├── interpreter.rs        # Interpreter implementation
│   │   ├── baseline_compiler.rs  # Baseline compiler implementation
│   │   ├── optimizing_compiler.rs # Optimizing compiler implementation
│   │   ├── execution_engine.rs   # Execution engine implementation
│   │   └── hotness_detector.rs   # Hotness detector implementation
│   ├── gc/             # Garbage collection system
│   │   ├── mod.rs      # GC module definition
│   │   ├── generational.rs  # Generational garbage collector
│   │   ├── incremental.rs   # Incremental garbage collector
│   │   └── concurrent.rs    # Concurrent garbage collector
│   └── profiler/       # Performance analysis and debugging tools
│       ├── mod.rs      # Tools module definition
│       ├── profiler.rs      # Profiler implementation
│       ├── debugger.rs       # Debugger implementation
│       └── visualizer.rs     # Visualization tool implementation
├── tests/              # Test suite
│   ├── ffi_test.rs     # FFI tests
│   ├── gc_test.rs      # Garbage collector tests
│   ├── jit_test.rs     # JIT compiler tests
│   ├── vm_test.rs      # Virtual machine tests
│   ├── runtime_test.rs # Runtime tests
│   ├── performance_test.rs # Performance tests
│   └── optimization_test.rs # Optimization tests
└── Cargo.toml          # Dependency configuration
```

## 🛠️ Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run specific test
cargo test gc_test

# Build in release mode
cargo build --release
```

## 📚 Core Modules

### Frontend
- **Lexical Analysis**: Converts source code to token stream
- **Syntax Analysis**: Builds abstract syntax tree (AST)
- **Semantic Analysis**: Checks types, scopes, etc.

### Virtual Machine
- **Register-based**: Uses 16 general-purpose registers
- **Instruction Set**: Load/store, arithmetic, comparison, logic, control flow, method calls
- **Execution Environment**: Manages global, local, instance, and class variables

### Garbage Collection
- **Mark-Sweep Algorithm**: Marks root objects, sweeps unmarked objects
- **Memory Management**: Tracks memory usage, automatically reclaims unused objects
- **Root Objects**: Global variables, local variables, instance variables, class variables

### JIT Compiler
- **Just-In-Time Compilation**: Compiles hot code paths to machine code
- **Optimization**: Type inference, inlining, loop optimization, etc.
- **Performance**: Significantly faster execution for hot code

### FFI Interface
- **Dynamic Library Loading**: Loads external C libraries
- **Function Calling**: Calls C functions and handles return values
- **Type Conversion**: Converts between Ruby values and C types

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve Rusty Ruby.

## 🌟 Future Development

- **Complete Ruby Language Support**: Implement all Ruby syntax features
- **Advanced Optimization**: More complex JIT optimizations, inline caching, escape analysis
- **Multi-platform Support**: Support for more operating systems and hardware architectures
- **Debugging Tools**: Enhanced debugger, performance analysis tools
- **Garbage Collection Optimization**: More efficient GC algorithms
- **Concurrency Support**: Better concurrent processing capabilities
- **Ecosystem**: Complete Ruby ecosystem, including standard library and gems
