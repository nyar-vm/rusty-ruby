# WASI UDF 扩展

有时你需要执行一些数据库原生不支持的复杂计算（如特定的加解密算法、复杂的文本分析或专有的业务模型）。RBQ 允许你通过 **WebAssembly Component Model (WASI)** 扩展引擎的能力。

## 1. 什么是 WASI UDF？

UDF (User Defined Function) 即用户自定义函数。RBQ 选择 WASI 作为扩展标准的原因是：
*   **沙箱隔离**：代码运行在安全的沙箱中，不会导致主进程崩溃。
*   **跨语言**：你可以用 Rust, C++, Go 或 Zig 编写算法。
*   **极致性能**：接近原生的运行速度。

## 2. 定义外部函数

在 RBQ 脚本中声明外部组件。

```rbq
@wasi(component = "crypto_utils.wasm", export = "encrypt-v1")
fn encrypt_email(data: utf8, key: utf8) -> utf8;
```

## 3. 在查询中使用

一旦定义，你就可以像使用内置函数一样使用它。

```rbq
users.map {
    id = $id
    secret_email = encrypt_email($email, "my-salt-123")
}
```

## 4. 部署与发现

RBQ 引擎会自动从指定的 `components/` 目录加载对应的 `.wasm` 文件。这些文件会被预编译并缓存在内存中。

---

**小结**：
*   不要在数据库里写存储过程，写 WASI 组件。
*   安全、高性能、可移植。
*   让数据库具备处理复杂算法的能力。

**下一阶段**：恭喜你走到了这里。如果你想进一步了解 RBQ 的运行机理，请参考 **[维护指南](../maintenance/index.md)**。
