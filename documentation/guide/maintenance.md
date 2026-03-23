# Rusty Ruby 开发和维护指南

## 1. 概述

本指南提供了 Rusty Ruby 项目的开发和维护流程，包括开发环境设置、代码风格、测试流程、版本控制和发布流程等内容。本指南旨在帮助开发人员快速上手项目，确保代码质量和项目的可持续发展。

## 2. 开发环境设置

### 2.1 系统要求

- **操作系统**：Windows、macOS 或 Linux
- **Rust 版本**：1.70.0 或更高
- **Node.js**（可选）：用于前端工具和测试
- **Git**：版本控制系统

### 2.2 安装步骤

1. **安装 Rust**
   - 访问 [rustup.rs](https://rustup.rs/) 下载并安装 Rust
   - 验证安装：`rustc --version`

2. **克隆仓库**
   ```bash
   git clone https://github.com/rusty-ruby/rusty-ruby.git
   cd rusty-ruby
   ```

3. **安装依赖**
   ```bash
   pnpm install
   ```

4. **构建项目**
   ```bash
   cargo build
   ```

5. **运行测试**
   ```bash
   cargo test
   ```

### 2.3 开发工具推荐

- **IDE**：Visual Studio Code、IntelliJ IDEA with Rust plugin
- **编辑器插件**：rust-analyzer、Ruby LSP
- **调试工具**：LLDB、GDB
- **性能分析工具**：perf、flamegraph

## 3. 代码风格

### 3.1 Rust 代码风格

Rusty Ruby 项目遵循 Rust 的官方代码风格指南，使用 `rustfmt` 进行代码格式化。

#### 3.1.1 代码格式化

- 使用 `cargo fmt` 格式化所有 Rust 代码
- 配置文件：`rustfmt.toml`

#### 3.1.2 代码规范

- 变量和函数名使用蛇形命名法（snake_case）
- 类型和 trait 名使用驼峰命名法（CamelCase）
- 常量使用全大写加下划线（SCREAMING_SNAKE_CASE）
- 缩进使用 4 个空格
- 每行代码不超过 100 个字符
- 使用 `#![warn(missing_docs)]` 确保所有公共 API 都有文档注释

### 3.2 Ruby 代码风格

对于 Ruby 代码，项目遵循 Ruby 社区的标准风格指南。

#### 3.2.1 代码规范

- 变量和方法名使用蛇形命名法（snake_case）
- 类和模块名使用驼峰命名法（CamelCase）
- 常量使用全大写加下划线（SCREAMING_SNAKE_CASE）
- 缩进使用 2 个空格
- 每行代码不超过 80 个字符

## 4. 测试流程

### 4.1 测试类型

Rusty Ruby 项目包含多种类型的测试：

- **单元测试**：测试单个组件的功能
- **集成测试**：测试多个组件的交互
- **性能测试**：测试代码的性能特性
- **语言规范测试**：测试 Ruby 语言规范的兼容性

### 4.2 运行测试

#### 4.2.1 运行所有测试

```bash
cargo test
```

#### 4.2.2 运行特定测试

```bash
# 运行单元测试
cargo test --lib

# 运行集成测试
cargo test --test integration

# 运行性能测试
cargo test --test performance

# 运行特定测试文件
cargo test --test <test_file>

# 运行特定测试函数
cargo test <test_function>
```

### 4.3 测试覆盖率

项目使用 `grcov` 工具来生成测试覆盖率报告：

```bash
# 安装 grcov
cargo install grcov

# 运行测试并生成覆盖率报告
RUSTFLAGS="-C instrument-coverage" cargo test
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing -o ./target/coverage

# 查看覆盖率报告
open ./target/coverage/index.html
```

### 4.4 测试最佳实践

- 为每个公共函数和模块编写单元测试
- 测试边界情况和错误处理
- 使用测试夹具（fixtures）来提供测试数据
- 保持测试代码简洁明了
- 测试应该是独立的，不依赖于其他测试的执行顺序

## 5. 版本控制

### 5.1 Git 工作流

Rusty Ruby 项目使用 Git 进行版本控制，遵循以下工作流：

1. **主分支**（main）：包含稳定的、已发布的代码
2. **开发分支**（develop）：包含正在开发的代码
3. **特性分支**（feature/*）：用于开发新特性
4. **修复分支**（fix/*）：用于修复 bug

### 5.2 提交规范

项目使用语义化提交规范，提交信息格式如下：

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### 5.2.1 类型（type）

- **feat**：新特性
- **fix**：bug 修复
- **docs**：文档更新
- **style**：代码风格调整
- **refactor**：代码重构
- **test**：测试相关
- **chore**：构建或依赖更新

#### 5.2.2 范围（scope）

范围指的是修改的模块或组件，例如：

- **compiler**：编译器相关
- **runtime**：运行时相关
- **lsp**：LSP 相关
- **tools**：工具相关

#### 5.2.3 示例

```
feat(compiler): add JIT compilation support

Add baseline JIT compiler implementation with hotness detection.

Fixes #123
```

### 5.3 分支管理

- **创建分支**：从 develop 分支创建新的特性或修复分支
- **提交代码**：在分支上进行开发，定期提交代码
- **推送分支**：将分支推送到远程仓库
- **创建 PR**：当功能完成后，创建 Pull Request 到 develop 分支
- **代码审查**：团队成员进行代码审查
- **合并分支**：审查通过后，将分支合并到 develop 分支
- **发布**：当 develop 分支稳定后，合并到 main 分支并发布新版本

## 6. 发布流程

### 6.1 版本号规范

项目使用语义化版本号（Semantic Versioning），格式为：

```
MAJOR.MINOR.PATCH
```

- **MAJOR**：不兼容的 API 变更
- **MINOR**：向后兼容的新特性
- **PATCH**：向后兼容的 bug 修复

### 6.2 发布步骤

1. **更新版本号**：在 `Cargo.toml` 文件中更新版本号
2. **更新 CHANGELOG**：记录本次发布的变更内容
3. **运行测试**：确保所有测试通过
4. **构建发布版本**：`cargo build --release`
5. **创建发布标签**：`git tag v<version>`
6. **推送标签**：`git push origin v<version>`
7. **发布到 crates.io**：`cargo publish`
8. **创建 GitHub Release**：在 GitHub 上创建新的 Release，包含发布说明和二进制文件

### 6.3 发布检查清单

- [ ] 所有测试通过
- [ ] 代码覆盖率达到目标
- [ ] 文档更新完成
- [ ] CHANGELOG 更新完成
- [ ] 版本号更新完成
- [ ] 构建成功
- [ ] 发布标签创建完成
- [ ] GitHub Release 创建完成
- [ ] crates.io 发布完成

## 7. 贡献流程

### 7.1 贡献指南

1. **Fork 仓库**：在 GitHub 上 fork 项目仓库
2. **克隆仓库**：克隆你的 fork 到本地
3. **创建分支**：从 develop 分支创建新的分支
4. **开发**：实现功能或修复 bug
5. **测试**：确保代码通过所有测试
6. **提交**：使用语义化提交规范提交代码
7. **推送**：将分支推送到你的 fork
8. **创建 PR**：在 GitHub 上创建 Pull Request 到原仓库的 develop 分支
9. **代码审查**：回应审查意见，进行必要的修改
10. **合并**：一旦审查通过，你的贡献将被合并

### 7.2 贡献类型

- **新特性**：实现新的功能或改进现有功能
- **bug 修复**：修复代码中的 bug
- **文档**：改进或更新文档
- **测试**：添加或改进测试
- **性能优化**：提高代码性能
- **代码重构**：改进代码结构和可读性

### 7.3 代码审查指南

- **代码质量**：代码是否符合项目的代码风格和质量标准
- **功能正确性**：代码是否正确实现了预期功能
- **测试覆盖**：是否为新代码添加了测试
- **文档**：是否为新功能或修改添加了文档
- **性能**：代码是否有性能问题
- **安全性**：代码是否有安全问题

## 8. 常见开发任务

### 8.1 添加新功能

1. **创建特性分支**：`git checkout -b feature/<feature-name>`
2. **实现功能**：编写代码实现新功能
3. **添加测试**：为新功能添加测试
4. **运行测试**：确保所有测试通过
5. **提交代码**：使用语义化提交规范提交代码
6. **推送分支**：`git push origin feature/<feature-name>`
7. **创建 PR**：在 GitHub 上创建 Pull Request

### 8.2 修复 bug

1. **创建修复分支**：`git checkout -b fix/<bug-name>`
2. **定位问题**：找到 bug 的根本原因
3. **修复问题**：编写代码修复 bug
4. **添加测试**：为修复添加测试，确保 bug 不会再次出现
5. **运行测试**：确保所有测试通过
6. **提交代码**：使用语义化提交规范提交代码
7. **推送分支**：`git push origin fix/<bug-name>`
8. **创建 PR**：在 GitHub 上创建 Pull Request

### 8.3 性能优化

1. **分析性能**：使用性能分析工具找出性能瓶颈
2. **优化代码**：针对性能瓶颈进行优化
3. **基准测试**：使用基准测试验证优化效果
4. **运行测试**：确保所有测试通过
5. **提交代码**：使用语义化提交规范提交代码
6. **推送分支**：`git push origin feature/<optimization-name>`
7. **创建 PR**：在 GitHub 上创建 Pull Request

### 8.4 文档更新

1. **创建文档分支**：`git checkout -b docs/<doc-name>`
2. **更新文档**：修改或添加文档内容
3. **预览文档**：确保文档格式正确，内容清晰
4. **提交代码**：使用语义化提交规范提交代码
5. **推送分支**：`git push origin docs/<doc-name>`
6. **创建 PR**：在 GitHub 上创建 Pull Request

## 9. 故障排查

### 9.1 常见问题

| 问题 | 可能原因 | 解决方案 |
|------|---------|---------|
| 构建失败 | 依赖问题 | 运行 `cargo update` 更新依赖 |
| 测试失败 | 代码逻辑错误 | 检查测试失败的具体原因，修复代码 |
| 性能问题 | 算法效率低 | 使用性能分析工具找出瓶颈，优化代码 |
| 内存泄漏 | 资源未正确释放 | 使用内存分析工具检测泄漏，修复代码 |
| 编译错误 | 语法错误或类型错误 | 检查编译器错误信息，修复代码 |

### 9.2 调试技巧

- **使用日志**：在代码中添加日志语句，了解程序的执行流程
- **使用调试器**：使用 LLDB 或 GDB 进行交互式调试
- **使用断言**：在关键位置添加断言，确保代码的正确性
- **使用单元测试**：编写单元测试来隔离和测试特定功能
- **使用性能分析工具**：使用 perf、flamegraph 等工具分析性能问题

## 10. 代码质量保证

### 10.1 静态分析

项目使用以下工具进行静态分析：

- **clippy**：Rust 代码 linting 工具
- **rustfmt**：代码格式化工具
- **cargo-audit**：依赖安全审计工具

#### 10.1.1 运行静态分析

```bash
# 运行 clippy
cargo clippy

# 运行 rustfmt
cargo fmt --check

# 运行 cargo-audit
cargo audit
```

### 10.2 代码审查

- **自审查**：在提交代码前，自己审查代码是否符合项目规范
- **同伴审查**：团队成员互相审查代码，确保代码质量
- **自动化审查**：使用 CI/CD 工具自动运行静态分析和测试

### 10.3 持续集成

项目使用 CI/CD 工具（如 GitHub Actions）进行持续集成，包括：

- **构建**：自动构建项目
- **测试**：自动运行所有测试
- **静态分析**：自动运行静态分析工具
- **代码覆盖率**：自动生成代码覆盖率报告
- **发布**：自动发布新版本（在特定条件下）

## 11. 性能分析和优化

### 11.1 性能分析工具

- **cargo-flamegraph**：生成火焰图，可视化程序的执行时间分布
- **perf**：Linux 系统性能分析工具，提供详细的性能统计信息
- **cargo-profiler**：Rust 代码性能分析工具
- **内置性能分析器**：位于 `compilers/ruby/src/profiler/` 目录

### 11.2 性能分析流程

1. **运行性能分析**：使用性能分析工具收集性能数据
2. **分析结果**：识别性能瓶颈
3. **优化代码**：针对瓶颈进行优化
4. **验证优化**：再次运行性能分析，验证优化效果

### 11.3 常见性能问题及解决方案

- **内存泄漏**：使用内存分析工具检测泄漏，修复代码
- **热点代码**：优化算法，减少循环次数，使用缓存
- **I/O 瓶颈**：使用异步 I/O，批量读写操作
- **方法调用开销**：内联频繁调用的方法，减少动态调度

## 12. 总结

本指南提供了 Rusty Ruby 项目的开发和维护流程，包括开发环境设置、代码风格、测试流程、版本控制、发布流程和贡献流程等内容。遵循这些指南将有助于确保项目的代码质量和可持续发展，同时帮助新开发人员快速上手项目。

通过不断改进和完善开发流程，Rusty Ruby 项目将能够更好地满足用户需求，提供更高效、更可靠的 Ruby 编译和运行环境。