# Ruby IR

Intermediate Representation for Ruby code in Rusty Ruby, providing a foundation for optimization and code generation.

## 🎯 Project Overview

Ruby-IR is a crate that provides the intermediate representation (IR) for Ruby code compilation in the Rusty Ruby project. It defines the data structures and traversal utilities for representing Ruby code in a form suitable for optimization and code generation.

## 🌟 Key Features

- **Abstract Syntax Tree (AST) Representation**: Structured representation of Ruby code
- **Traversal Utilities**: Tools for navigating and manipulating IR nodes
- **Serialization Support**: Convert IR objects to and from serialized formats
- **Optimization Framework**: Foundation for code optimization passes
- **Integration**: Seamlessly works with other Rusty Ruby components

## 🚀 Quick Start

```rust
use ruby_ir::*;

// Create IR nodes
let expression = Expression::Constant(Constant::Integer(42));

// Traverse and manipulate IR
struct MyVisitor;

impl Visitor for MyVisitor {
    fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Constant(constant) => println!("Found constant: {:?}", constant),
            _ => {}
        }
    }
}

let mut visitor = MyVisitor;
visitor.visit_expression(&expression);
```

## 🏗️ Architecture

### Core Components

- **Expressions**: Represent Ruby expressions like constants, variables, method calls, etc.
- **Statements**: Represent Ruby statements like assignments, conditionals, loops, etc.
- **Symbols**: Represent Ruby symbols and identifiers
- **Types**: Represent Ruby types and type information
- **Traversal**: Utilities for visiting and manipulating IR nodes

### Optimization Passes

- **Constant Folding**: Evaluate constant expressions at compile time
- **Dead Code Elimination**: Remove unreachable code
- **Inline Expansion**: Inline method calls for better performance
- **Type Inference**: Infer types for variables and expressions

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

### Basic IR Creation

```rust
use ruby_ir::*;

// Create a binary expression: 1 + 2
let left = Expression::Constant(Constant::Integer(1));
let right = Expression::Constant(Constant::Integer(2));
let add_expr = Expression::BinaryOp(
    BinaryOp::Add,
    Box::new(left),
    Box::new(right)
);

// Create an assignment statement
let variable = Expression::Variable("result".to_string());
let assignment = Statement::Assignment(
    Box::new(variable),
    Box::new(add_expr)
);

// Create a program
let program = Program {
    statements: vec![assignment]
};
```

### IR Traversal

```rust
use ruby_ir::*;

struct ConstantCollector {
    constants: Vec<Constant>
}

impl Visitor for ConstantCollector {
    fn visit_constant(&mut self, constant: &Constant) {
        self.constants.push(constant.clone());
    }
}

let mut collector = ConstantCollector { constants: vec![] };
collector.visit_program(&program);

println!("Found constants: {:?}", collector.constants);
```

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve Ruby-IR.
