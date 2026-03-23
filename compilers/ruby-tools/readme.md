# Rusty Ruby Tools

A **pure Rust** collection of Ruby language tools built for the Rusty Ruby ecosystem.

## 🎯 Project Overview

Ruby-Tools is a comprehensive set of Ruby language tools built entirely in Rust. These tools provide a familiar Ruby development experience while leveraging Rust's safety and performance benefits.

## 🌟 Key Features

- **Pure Rust Implementation**: 🦀 Built entirely in Rust for maximum safety and performance
- **Ruby Command**: 📝 Execute Ruby scripts with the `ruby` command
- **Interactive Ruby (IRB)**: 💬 Interactive Ruby shell for experimentation
- **Gem Management**: 📦 Basic `gem` command for Ruby package management
- **Rake Task Runner**: 🏃 `rake` command for task automation
- **Documentation Tools**: 📚 `rdoc` and `ri` for generating and viewing documentation
- **Test Runner**: 🧪 `testrb` for running Ruby tests

## 🚀 Quick Start

```bash
# Execute a Ruby script
ruby script.rb

# Start interactive Ruby shell
irb

# Run a Rake task
rake build

# Generate documentation
rdoc

# Run tests
testrb test_file.rb
```

## 🏗️ Architecture

### Tools Overview

- **ruby**: Main Ruby interpreter and script executor
- **irb**: Interactive Ruby shell for experimentation and debugging
- **gem**: Package manager for Ruby libraries and applications
- **rake**: Task runner for automating development tasks
- **rdoc**: Documentation generator for Ruby code
- **ri**: Documentation viewer for Ruby APIs
- **testrb**: Test runner for Ruby test suites

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

### Ruby Command

```bash
# Execute a Ruby script
ruby hello.rb

# Run with verbose output
ruby -v hello.rb

# Execute inline Ruby code
ruby -e "puts 'Hello, World!'"
```

### Interactive Ruby (IRB)

```bash
# Start IRB
irb

# Start IRB with verbose output
irb -v

# Execute a Ruby file and enter IRB
irb -r hello.rb
```

### Rake Task Runner

```bash
# List available tasks
rake -T

# Run a specific task
rake build

# Run tasks with verbose output
rake -v test
```

### Documentation Tools

```bash
# Generate documentation for a project
rdoc

# Generate documentation with options
rdoc --main README.md --title "My Project"

# View documentation for a class or method
ri String
ri String#length
```

### Test Runner

```bash
# Run a test file
testrb test_file.rb

# Run tests with verbose output
testrb -v test_file.rb

# Run tests in a directory
testrb test/
```

## 🤝 Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to help improve Ruby-Tools.
