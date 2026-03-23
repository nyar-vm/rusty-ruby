# 编译器内部实现与扩展机制

RBQ 的强大不仅源于其简洁的语法，更源于其深层的编译器管线与高度解耦的扩展机制。本章将深入探讨 RBQ 如何将逻辑意图转化为物理执行和 RPC 服务，以及如何通过扩展点增强其功能。

## 概述

RBQ 编译器实现了 xRPC 概念，将 DSL 定义转换为高性能的 Rust 代码，包含 ORM 和 RPC 功能。编译器采用三层 IR 管线，确保从高级建模到物理执行和 RPC 服务的平滑过渡与极致优化。

## 1. 编译器中间表示 (IR) 管线

RBQ 采用三层 IR 管线，确保了从高级建模到物理执行和 RPC 服务的平滑过渡与极致优化。

### 1.1 HIR (High-level IR): 高级语义层
*   **职责**: 处理语法解析、语义检查、类型推导。
*   **特性**: 
    *   **Trait 展平 (Flattening)**: 将 `trait` 定义的字段合并到目标 `model` 中。
    *   **Micro 函数内联**: 将 `micro` 纯逻辑算子直接展开到调用点。
    *   **脱糖**: 处理尾随闭包等语法糖。
    *   **RPC 服务解析**: 解析 `service` 定义，生成对应的服务 trait。

### 1.2 MIR (Middle IR): 逻辑执行计划层
*   **职责**: 将建模概念抽象为数据库逻辑对象（Table, Column, Relation, Query Plan）和 RPC 服务定义。
*   **优化**: 
    *   **谓词下推 (Predicate Pushdown)**: 将过滤条件尽可能移向数据源。
    *   **列修剪 (Column Pruning)**: 仅读取查询所需的字段。
    *   **逻辑关联映射**: 将 `&T` 和 `list<&T>` 转换为逻辑 Join 关系。
    *   **RPC 方法映射**: 将服务方法映射为对应的 RPC 处理函数。

### 1.3 LIR (Low-level IR): 物理执行计划层
*   **职责**: 将逻辑计划翻译为特定数据库方言（SQL AST 或驱动 API）和 RPC 代码。
*   **特性**:
    *   **自动 Join 生成**: 根据关系路径自动推导 Join 条件。
    *   **参数绑定**: 确保所有查询使用参数化形式，杜绝 SQL 注入。
    *   **方言适配**: 处理不同数据库在分页、锁定、聚合等语法上的差异。
    *   **RPC 代码生成**: 生成客户端和服务器代码，包括流处理逻辑。

---

## 2. 类型映射系统 (RBQ -> RBQ Value)

RBQ 定义了一套标准的中转类型，确保了跨引擎的数据一致性。

| RBQ 类型 | 映射方言 (示例) | RBQ Value 类型 | 说明 |
| :--- | :--- | :--- | :--- |
| `i32` / `i64` | `INT` / `BIGINT` | `Int32` / `Int64` | 标准整数 |
| `f64` | `DOUBLE` | `Float64` | 浮点数 |
| `string` | `VARCHAR` / `TEXT` | `Text` | 强制 UTF-8 编码文本 |
| `bytes` | `BYTEA` / `BLOB` | `Bytes` | 二进制数据 |
| `datetime` | `TIMESTAMP WITH TZ` | `DateTime` | 带时区的时间戳 |
| `bool` | `BOOLEAN` | `Bool` | 布尔值 |
| `&T` | `FK` (主键类型) | (主键类型) | 物理外键映射 |
| `[]T` | `ARRAY` / `JSON` | `Vec<T>` | 数组映射 |
| `map<K,V>` | `JSON` | `Map<K,V>` | 映射映射 |

---

## 3. 方言适配与引擎配置

RBQ 编译器通过 `DialectAdapter` 处理不同数据库的语法差异，并通过 `EngineType` 枚举定义支持的后端偏好。物理数据库的连接细节（如 URL、连接池）由外部配置文件（如 `rbq.toml`）统一管理。

### 3.1 DialectAdapter 接口 (Rust)
这是驱动层需要实现的核心接口：

```rust
trait DialectAdapter {
    # 类型映射：将 RBQ 类型映射为方言 DDL 字符串
    fn map_type(&self, t: &RbqType) -> String;
    
    # DDL 生成 (用于迁移)
    fn create_table(&self, table: &MirTable) -> String;
    fn add_column(&self, col: &MirColumn) -> String;
    
    # DML 生成 (用于查询)
    fn select_statement(&self, query: &MirQuery) -> String;
    fn insert_statement(&self, insert: &MirInsert) -> String;
    
    # 标识符引用 (如 PG 的 "", MySQL 的 ``)
    fn quote_identifier(&self, ident: &str) -> String;
}
```

### 3.2 支持的引擎类型 (EngineType)
```rbq
union EngineType {
  PostgreSQL { version: String?, extensions: Set<String>? },
  MySQL { version: String?, engine: String?, charset: String? },
  SQLite { version: String? },
  Redis { search_enabled: bool? },
  CloudflareD1 { region: String? },
  CSV { directory: String },
  Memory { persist_path: String? },
  WASI { version: String }, # 运行时扩展
  Custom { adapter: String, config: Map<String, Value> }
}
```

### 3.3 核心特性实现矩阵

| 特性 | PostgreSQL | MySQL | SQLite | Redis |
| :--- | :--- | :--- | :--- | :--- |
| **ADT Union** | `JSONB` + GIN | `JSON` + 虚拟列 | `TEXT` (JSON) | `Hash` / `ReJSON` |
| **Computed** | `GENERATED` | `VIRTUAL` | `GENERATED` | 应用层计算 |
| **Vector** | `pgvector` | 原生 `VECTOR` | `sqlite-vss` | `RediSearch` |
| **Soft Delete** | 自动视图过滤 | SQL 注入过滤 | SQL 注入过滤 | 自动过滤 |

---

## 4. 扩展机制 (Extension Mechanism)

RBQ 区分“业务逻辑层扩展”和“驱动执行层扩展”，以平衡业务灵活性与底层性能。

### 4.1 逻辑层扩展 (Logic Extensions)
*   **编译时注解**: 如 `@audit`, `@soft_delete`。编译器在生成 LIR 时自动注入相关逻辑。
*   **运行时钩子 (Hooks)**: 
    *   `@before_save`: 保存前触发（如密码加密）。
    *   `@after_save`: 保存后触发（如发送消息通知）。
*   **RPC 中间件**: 通过 `@middleware` 注解，为 RPC 服务添加中间件。

### 4.2 驱动层扩展 (Driver Extensions)
通过 `DriverExtension` 接口在物理执行路径上插入拦截逻辑：
*   **before_connect / after_connect**: 处理多租户上下文初始化（如 `SET tenant_id`）。
*   **intercept_query**: 慢查询监控、审计日志、性能指标收集。

---

## 5. 迁移与自动降级

*   **迁移生成**: RBQ 比较 MIR 差异，生成带事务保护的 DDL 变更脚本。
*   **自动降级 (Downgrade)**: 当目标库不支持高级特性（如原生 JSONB）时，后端会自动将其降级为等价的物理表示（如 TEXT），并由 Runtime 补偿逻辑。

---

## 6. xRPC 代码生成

编译器 `rbqc` 为 RPC 服务生成以下代码：

1. **服务 trait**: 定义服务接口，包含所有方法签名。
2. **客户端实现**: 生成客户端代码，处理网络通信和序列化。
3. **服务器实现**: 生成服务器骨架，处理请求分发和响应。
4. **流类型**: 生成流类型的包装，支持流式 RPC。

### 6.1 生成代码示例

```rust
// 服务 trait
#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError>;
    async fn list_users(&self, req: ListUsersRequest) -> Result<impl Stream<Item = User>, RpcError>;
}

// 客户端
struct UserServiceClient {
    inner: xrpc::client::Client,
}

impl UserServiceClient {
    async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError> {
        self.inner.unary("/user.UserService/get_user", req).await
    }
}

// 服务器骨架
fn serve_user_service<S: UserService>(service: S) -> xrpc::Server {
    // 注册方法处理器
}
```

---

## 7. 恭喜你！

你已经完成了 RBQ 编译器内部实现的全部课程。现在，你已经具备了理解 RBQ 内部工作原理和扩展 RBQ 功能的知识。

**下一步建议：**
*   **[RBQ CLI 工具指南](../guide/commands/index.md)**：学习如何进行生产环境的迁移与备份。
*   **[数据库驱动设计与新增指南](database/index.md)**：掌握如何为 RBQ 增加新的数据库支持。
