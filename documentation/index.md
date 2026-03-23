# Rusty Ruby 文档索引

## 1. 项目概览

Rusty Ruby 是一个用 Rust 实现的 Ruby 编译器和运行时环境，采用现代编译技术和内存管理策略。本文档提供了项目的设计、开发和维护指南，帮助开发人员理解项目结构、参与开发和维护工作。

## 2. 文档结构

### 2.1 设计文档

| 文档名称 | 描述 | 路径 |
|---------|------|------|
| 架构设计 | 项目的整体架构和核心组件 | [design/architecture.md](design/architecture.md) |

### 2.2 指南文档

| 文档名称 | 描述 | 路径 |
|---------|------|------|
| 开发和维护指南 | 开发流程、代码风格和维护指南 | [guide/maintenance.md](guide/maintenance.md) |
| 解释器改良指南 | 如何定位性能问题、改良解释器 | [guide/interpreter.md](guide/interpreter.md) |
| 验证和回归测试指南 | 如何验证修改和做回归测试 | [guide/regression.md](guide/regression.md) |

### 2.3 优化文档

| 文档名称 | 描述 | 路径 |
|---------|------|------|
| 性能优化指南 | 详细的性能优化策略和实践 | [optimization/index.md](optimization/index.md) |

## 3. 项目结构

```
rusty-ruby/
├── compilers/         # 编译器相关代码
│   ├── ruby/          # Ruby 编译器实现
│   ├── ruby-ir/       # Ruby IR 实现
│   ├── ruby-lsp/      # Ruby LSP 实现
│   ├── ruby-macros/   # Ruby 宏实现
│   ├── ruby-tools/    # Ruby 工具实现
│   ├── ruby-types/    # Ruby 类型定义
│   └── ruby-wasi/     # Ruby WASI 实现
├── examples/          # 示例代码
├── documentation/     # 项目文档
│   ├── design/        # 设计文档
│   ├── guide/         # 指南文档
│   ├── optimization/  # 优化相关文档
│   └── index.md       # 文档索引
└── frontends/         # 前端相关代码
```

## 4. 快速导航

### 4.1 设计文档

- [架构设计](design/architecture.md) - 了解项目的整体架构和核心组件

### 4.2 指南文档

- [开发和维护指南](guide/maintenance.md) - 学习项目的开发流程和最佳实践
- [解释器改良指南](guide/interpreter.md) - 了解如何定位性能问题和改良解释器
- [验证和回归测试指南](guide/regression.md) - 学习如何验证修改和做回归测试

### 4.3 优化文档

- [性能优化指南](optimization/index.md) - 掌握性能优化技术和策略

## 5. 模块指南

### 5.1 编译器模块

- **ruby/**: Ruby 编译器的核心实现，包括多层级编译架构、垃圾收集系统、性能分析和调试工具等
- **ruby-ir/**: Ruby 中间表示的实现，用于优化和代码生成
- **ruby-lsp/**: Ruby 语言服务器协议实现，提供代码补全、格式化等功能
- **ruby-macros/**: Ruby 宏实现，用于编译期代码生成
- **ruby-tools/**: Ruby 工具实现，包括 gem、irb、rake 等
- **ruby-types/**: Ruby 类型定义，用于整个项目的类型系统
- **ruby-wasi/**: Ruby WASI 实现，用于 WebAssembly 支持

## 6. 开发流程

1. **环境设置**: 安装 Rust 和必要的依赖
2. **代码修改**: 在相应的模块中进行代码修改
3. **测试**: 运行测试确保修改不会破坏现有功能
4. **文档更新**: 更新相关文档以反映修改
5. **提交**: 提交代码和文档修改
6. **代码审查**: 团队成员进行代码审查
7. **合并**: 审查通过后，将代码合并到主分支

## 7. 常见问题

### 7.1 如何开始开发？

参考 [开发和维护指南](guide/maintenance.md) 中的开发环境设置部分，安装必要的依赖并设置开发环境。

### 7.2 如何运行测试？

使用 `cargo test` 命令运行所有测试，或使用 `cargo test --test <test_file>` 运行特定的测试文件。

### 7.3 如何优化性能？

参考 [性能优化指南](optimization/index.md) 中的性能优化技术部分，使用性能分析工具识别瓶颈并进行优化。

### 7.4 如何改良解释器？

参考 [解释器改良指南](guide/interpreter.md) 中的方法，定位性能问题、确定问题归属模块、改良整个解释器。

### 7.5 如何验证修改？

参考 [验证和回归测试指南](guide/regression.md) 中的方法，确保修改的正确性，避免引入新的问题。

## 8. 联系与支持

- **项目仓库**: [https://github.com/rusty-ruby/rusty-ruby](https://github.com/rusty-ruby/rusty-ruby)
- **问题跟踪**: [https://github.com/rusty-ruby/rusty-ruby/issues](https://github.com/rusty-ruby/rusty-ruby/issues)
- **讨论区**: [https://github.com/rusty-ruby/rusty-ruby/discussions](https://github.com/rusty-ruby/rusty-ruby/discussions)
