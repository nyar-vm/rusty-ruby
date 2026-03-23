# Ruby LSP

A Language Server Protocol (LSP) implementation for Ruby, providing modern IDE integration for Ruby development.

## 🎯 Project Overview

Ruby-LSP is a dedicated Language Server Protocol implementation for Ruby, built using the Oak LSP framework. It provides rich language features for Ruby development, including code completion, hover information, go-to definition, and more.

## 🌟 Key Features

- **Code Completion**: Intelligent suggestions for Ruby keywords, identifiers, and methods
- **Hover Information**: Context-aware documentation and type information for Ruby symbols
- **Go to Definition**: Navigate directly to the definition of classes, methods, and variables
- **Find References**: Locate all references to a symbol throughout the codebase
- **Code Diagnostics**: Real-time error reporting and code analysis
- **Syntax Highlighting**: Enhanced syntax highlighting for Ruby code
- **Code Formatting**: Automatic code formatting for consistent style

## 🚀 Quick Start

### Command Line

```bash
# Start the LSP server
ruby-lsp
```

### Editor Integration

#### VS Code

1. Install the [Ruby LSP](https://marketplace.visualstudio.com/items?itemName=Shopify.ruby-lsp) extension
2. Configure the extension to use the custom LSP server:

```json
{
  "rubyLsp.serverPath": "/path/to/rusty-ruby/target/release/ruby-lsp"
}
```

#### Vim/Neovim

Use a plugin like [coc.nvim](https://github.com/neoclide/coc.nvim) or [LanguageClient-neovim](https://github.com/autozimu/LanguageClient-neovim) and configure it to use the Ruby LSP server.

## 🏗️ Architecture

The Ruby LSP server is built using the following components:

- **Oak LSP**: Provides the core LSP server framework
- **Oak Ruby**: Provides Ruby parsing functionality
- **Oak VFS**: Provides virtual file system abstraction

### Project Structure

```
ruby-lsp/
├── bin/           # Binary entry point
│   └── main.rs    # Main server entry point
├── src/           # Source code
│   ├── lsp/       # LSP-specific functionality
│   │   ├── formatter/    # Code formatting
│   │   ├── highlighter/  # Syntax highlighting
│   │   └── mod.rs        # LSP module definition
│   └── lib.rs     # Main library implementation
├── Cargo.toml     # Cargo configuration
└── README.md      # This file
```

## 🛠️ Development

```bash
# Build the project
cargo build

# Run tests
cargo test

# Build in release mode
cargo build --release
```

## 📚 Language Features

### Code Completion

Ruby-LSP provides intelligent code completion for:
- Ruby keywords and built-in methods
- User-defined classes, methods, and variables
- Gem dependencies and their APIs
- Context-aware suggestions based on the current code

### Hover Information

When hovering over Ruby symbols, Ruby-LSP provides:
- Documentation for methods and classes
- Type information for variables and expressions
- Parameter hints for method calls

### Go to Definition

Ruby-LSP allows you to navigate to the definition of:
- Classes and modules
- Methods and functions
- Variables and constants
- Require statements and their dependencies

### Find References

Ruby-LSP can find all references to:
- Classes and modules
- Methods and functions
- Variables and constants
- Symbols and identifiers

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve Ruby-LSP.
