# 数据库驱动设计与新增指南

RBQ (Rust Business Query) 的核心目标是为不同的数据库提供一个统一、安全且高性能的执行接口。本文档介绍了 RBQ 驱动的统一设计原则，并为如何新增一个数据库驱动提供指导。

## 1. 统一设计原则

为了确保所有驱动在行为上的一致性，RBQ 遵循以下设计原则：

### 1.1 协议尊重 (Protocol Respect)
RBQ 驱动应当基于数据库的原生协议（如 MySQL 二进制协议、PostgreSQL V3 协议）实现。我们不倾向于使用通用的 ODBC/JDBC 桥接，而是追求极致的性能和对原生特性的支持。

### 1.2 参数绑定强制化 (Mandatory Parameter Binding)
所有的驱动必须通过参数化查询（Prepared Statements）来执行 SQL。
- **禁止**：在驱动层进行字符串拼接。
- **必须**：将 SQL 模板与参数（`Value` 数组）分开传输给数据库引擎。
这从根本上杜绝了 SQL 注入风险。

### 1.3 统一错误模型 (Unified Error Model)
驱动负责将底层协议产生的错误（ErrorCode, SQLSTATE）映射为 `rbq_types::RBQError`。
- 常见的映射包括：`ConnectionError`, `DatabaseError`, `AuthenticationError` 等。

### 1.4 状态透明化 (Transparent State)
驱动必须准确报告连接的 `ConnectionState`（如 `Idle`, `InTransaction`, `Busy`），以便连接池能够正确管理连接的生命周期。

## 2. 新增驱动指导

若要为 RBQ 增加一个新的数据库支持（例如 `we-trust-oracle`），请遵循以下步骤：

### 第一步：创建 Package
在 `yyds/packages/` 目录下创建一个新的 crate，命名为 `we-trust-<name>`。

### 第二步：实现 Connection Trait
实现 `rbq_core::Connection` 接口，这是驱动的核心。
```rust
pub trait Connection: Send + Sync {
    type Tx: Transaction;
    async fn execute(&self, query: &str, params: &[Value]) -> RBQResult<ExecuteResult>;
    async fn query(&self, query: &str, params: &[Value]) -> RBQResult<ResultSet>;
    // ... 其他方法 ...
}
```
**关键点**：
- 确保 `execute` 和 `query` 使用异步 IO。
- 正确处理参数绑定逻辑。

### 第三步：实现 DriverAdapter Trait
实现 `rbq_core::DriverAdapter`，负责连接的创建。
```rust
pub trait DriverAdapter: Send + Sync + 'static {
    type Conn: Connection;
    async fn connect(&self, config: &DbConfig) -> RBQResult<Self::Conn>;
    // ... 元数据方法 ...
}
```

### 第四步：定义类型映射
在驱动内部实现将数据库原生类型与 `rbq_types::Value` 之间的双向转换。

### 第五步：错误映射
编写一个辅助函数或实现 `From` trait，将底层的错误类型转换为 `RBQError`。

### 第六步：测试验证
- **单元测试**：测试协议解析、类型转换。
- **集成测试**：在 `tests/` 目录下编写针对真实数据库实例的连接和查询测试。

## 3. 最佳实践

- **零拷贝解析**：尽量使用 `bytes` 或 `nom` 等库进行高性能协议解析。
- **异步优先**：底层 IO 必须使用 `tokio` 或类似异步运行时。
- **扩展性**：通过 `DriverExtension` 为该数据库特有的功能（如 Redis 的 Pub/Sub）提供扩展支持。
