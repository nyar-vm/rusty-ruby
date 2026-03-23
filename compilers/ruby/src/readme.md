# Rusty Ruby 编译器架构

## 1. 架构概览

Rusty Ruby 是一个用 Rust 实现的 Ruby 语言编译器和运行时环境，采用现代编译技术和内存管理策略。

### 核心组件

- **前端**：词法分析、语法分析（基于 oak_ruby 库）
- **中间表示**：AST 到 IR 的转换
- **后端**：IR 到 VM 指令的转换
- **运行时**：基于寄存器的 VM、垃圾收集器、JIT 编译器、FFI 接口

### 编译器管道

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  源代码     │ -> │  词法分析   │ -> │  语法分析   │ -> │    AST      │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                                              │
                                              ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  执行结果   │ <- │   虚拟机    │ <- │  VM指令     │ <- │    IR       │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                      ↑     ↑
                      │     │
              ┌───────┴─────┴───────┐
              │                     │
       ┌──────▼──────┐      ┌───────▼──────┐
       │  JIT编译器  │      │  垃圾收集器  │
       └─────────────┘      └──────────────┘
```

## 2. 目录结构

```
compilers/ruby/
├── src/
│   ├── lib.rs          # 主入口，包含 Ruby 运行时环境
│   ├── vm.rs           # 基于寄存器的虚拟机实现
│   ├── gc.rs           # 垃圾收集器实现
│   ├── jit.rs          # JIT 编译器实现
│   └── codegen.rs      # 代码生成（AST到IR到VM指令）
├── examples/
│   └── ffi_example.rs  # FFI 使用示例
├── tests/
│   ├── ffi_test.rs     # FFI 测试
│   ├── gc_test.rs      # 垃圾收集器测试
│   ├── jit_test.rs     # JIT 编译器测试
│   ├── vm_test.rs      # 虚拟机测试
│   └── runtime_test.rs # 运行时测试
└── Cargo.toml          # 依赖配置
```

## 3. 核心模块详解

### 3.1 前端（Frontend）

- **词法分析**：使用 oak_ruby::RubyLexer 将源代码转换为 token 流
- **语法分析**：使用 oak_ruby::RubyParser 构建抽象语法树（AST）
- **语义分析**：检查类型、作用域等

### 3.2 中间表示（IR）

- 基于 oak_ruby 的 AST 结构
- 提供优化和代码生成的基础

### 3.3 后端（Backend）

- **代码生成**：将 AST 转换为 VM 指令
- **指令优化**：简单的指令优化

### 3.4 虚拟机（VM）

- **基于寄存器**：使用 16 个通用寄存器
- **指令集**：包含加载/存储、算术、比较、逻辑、控制流、方法调用等指令
- **执行环境**：管理全局变量、局部变量、实例变量、类变量等

### 3.5 垃圾收集器（GC）

- **标记-清除算法**：标记根对象，清除未标记对象
- **内存管理**：追踪内存使用，自动回收不再使用的对象
- **根对象**：全局变量、局部变量、实例变量、类变量

### 3.6 JIT 编译器

- **热点检测**：追踪指令执行次数
- **即时编译**：将热点代码编译为优化的执行路径
- **编译缓存**：缓存编译结果，避免重复编译

### 3.7 FFI 接口

- **动态库加载**：加载外部 C 库
- **函数调用**：调用 C 函数并处理返回值
- **类型转换**：在 Ruby 值和 C 类型之间转换

## 4. 使用示例

### 4.1 基本用法

```rust
use ruby::Ruby;

fn main() -> ruby::Result<()> {
    let mut ruby = Ruby::new()?;
    
    // 执行 Ruby 脚本
    ruby.execute_script("$result = 1 + 2 * 3")?;
    
    // 获取结果
    let result = ruby.get_global("$result")?;
    println!("Result: {:?}", result);
    
    Ok(())
}
```

### 4.2 FFI 示例

```rust
use ruby::Ruby;
use ruby::ffi;

fn main() -> ruby::Result<()> {
    let mut ruby = Ruby::new()?;
    
    // 加载动态库
    ffi::load_library(&mut ruby, "libexample.dll")?;
    
    // 获取 C 函数
    let func_key = ffi::get_function(&mut ruby, "libexample.dll", "add")?;
    
    // 调用 C 函数
    let result = ffi::call_function(&mut ruby, &func_key)?;
    println!("Result: {}", result);
    
    Ok(())
}
```

### 4.3 定义类和方法

```rust
use ruby::Ruby;
use ruby_types::RubyValue;

fn main() -> ruby::Result<()> {
    let mut ruby = Ruby::new()?;
    
    // 定义类
    ruby.define_class("Person")?;
    
    // 定义方法
    ruby.define_method("Person", "greet", Box::new(|| ()))?;
    
    Ok(())
}
```

## 5. 开发者指南

### 5.1 编译和测试

```bash
# 编译
cargo build

# 运行测试
cargo test

# 运行示例
cargo run --example ffi_example
```

### 5.2 扩展编译器

1. **添加新指令**：在 `vm.rs` 的 `Instruction` 枚举中添加新指令
2. **实现指令处理**：在 `VMState::execute` 方法中添加对应处理逻辑
3. **添加优化**：在 `jit.rs` 中添加新的优化策略
4. **扩展 FFI**：在 `lib.rs` 和 `vm.rs` 中添加对新类型的支持

### 5.3 性能优化

- **JIT 优化**：调整编译阈值，优化热点代码路径
- **GC 调优**：调整内存阈值，优化垃圾收集时机
- **指令优化**：添加更多指令级优化

## 6. 技术栈

- **Rust**：主要开发语言
- **oak_ruby**：词法分析和语法分析
- **libloading**：动态库加载
- **std::sync**：并发和同步原语

## 7. 未来发展

- **完整的 Ruby 语言支持**：实现所有 Ruby 语法特性
- **高级优化**：更复杂的 JIT 优化、内联缓存等
- **多平台支持**：支持更多操作系统和硬件架构
- **调试工具**：添加调试器、性能分析工具等

## 8. 贡献指南

- 遵循 Rust 代码风格
- 编写测试用例
- 提交 PR 前确保所有测试通过
- 文档更新与代码同步

## 9. 许可证

MIT License