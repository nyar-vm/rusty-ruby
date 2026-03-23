# 代码编译 (Compile)

`rbqc` 命令将 `.rbq` 模型定义转换为 Rust 代码，包括数据模型、数据库操作和 RPC 服务。

## 常用命令

### 基本编译
```bash
rbqc --config rbq.toml --output src
```

### 指定配置文件
```bash
rbqc --config path/to/config.toml --output src
```

## 参数说明

- `--config`, `-c`: 配置文件路径，包含数据库连接信息和编译选项。
- `--output`, `-o`: 输出目录。如果未指定，将根据配置或默认规则生成。

## 配置文件选项

`rbq.toml` 配置文件中与编译相关的选项：

### [project] 部分
- `source_dir`: RBQ 源文件目录，包含所有 `.rbq` 文件，默认为 "models"

### [databases] 部分
- `url`: 数据库连接字符串，格式根据数据库类型而定
- `schema`: 可选的数据库架构名称，如 PostgreSQL 的 "public"

### [generate] 部分
- `target`: 目标语言，支持 "rust" 或 "typescript"
- `output`: 生成代码的输出目录

### [generate.rust] 子部分
- `serde`: 是否为生成的类型添加 serde 序列化/反序列化支持，默认为 false
- `runtime`: 是否生成运行时支持代码，默认为 false

## 编译过程

编译器 `rbqc` 执行以下步骤：

1. **解析**：读取所有 `.rbq` 文件，构建 AST。
2. **链接**：根据 `using` 语句解析跨文件引用，构建全局符号表。
3. **验证**：检查类型、约束、循环依赖等。
4. **代码生成**：
   - 为每个文件生成 Rust 模块。
   - 生成模型结构体（`struct`）及序列化实现（使用 `serde`）。
   - 生成数据库操作层（如果绑定数据库）：
     - CRUD 方法。
     - 查询构建器。
     - 事务支持。
   - 生成 RPC 消息结构体。
   - 生成服务 trait 和客户端/服务器代码（基于 xRPC）。
   - 生成流类型的包装。

5. **输出**：将生成的文件写入 `output_dir`，并生成 `mod.rs` 统一导出。

## 编译产物

以 `user.rbq` 为例，生成代码结构：

```
src/user/
├── mod.rs                # 模块入口，重新导出公共项
├── models.rs              # 模型实体（User, ...）
├── db.rs                  # 数据库操作（UserOps, QueryBuilder, 事务）
├── rpc.rs                 # RPC 消息和服务定义（UserService trait, client, server）
└── streams.rs             # 流类型（如 UserStream, ChatMessageStream）
```

### 生成的模型实体 (`models.rs`)

```rust
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i64,
    username: String,
    email: String,
    created_at: DateTime<Utc>,
    // 关系字段（懒加载）
    posts: Option<Vec<Post>>,  // 需要导入 Post
}
```

### 生成的数据库操作 (`db.rs`)

```rust
use crate::db::Database;   // 自研 Database trait
use crate::models::User;

impl User {
    // 基础 CRUD
    async fn find_by_id(db: &impl Database, id: i64) -> Result<Option<Self>, DbError> {
        // 生成的参数化查询
    }

    async fn insert(&self, db: &impl Database) -> Result<Self, DbError> {
        // ...
    }

    // 查询构建器
    fn query() -> UserQueryBuilder {
        UserQueryBuilder::new()
    }
}

// 查询构建器
struct UserQueryBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    condition: Option<String>,  // 实际更复杂，使用表达式
}

impl UserQueryBuilder {
    fn filter<F>(mut self, f: F) -> Self where F: FnOnce(&UserExpr) -> Condition { ... }
    fn order_by<F>(mut self, f: F) -> Self where F: FnOnce(&UserExpr) -> OrderBy { ... }
    async fn fetch(self, db: &impl Database) -> Result<Vec<User>, DbError> { ... }
}
```

### 生成的 RPC 代码 (`rpc.rs`)

```rust
// 生成的请求/响应消息
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetUserRequest {
    id: i64,
}

// 服务 trait
#[async_trait]
trait UserService: Send + Sync + 'static {
    async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError>;
    async fn list_users(&self, req: ListUsersRequest) -> Result<impl Stream<Item = User>, RpcError>;
    // ...
}

// 客户端
struct UserServiceClient {
    inner: xrpc::client::Client,
}

impl UserServiceClient {
    async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError> {
        self.inner.unary("/user.UserService/get_user", req).await
    }
    // ...
}

// 服务器骨架
fn serve_user_service<S: UserService>(service: S) -> xrpc::Server {
    // 注册方法处理器
}
```

### 生成的流类型 (`streams.rs`)

```rust
type UserStream = xrpc::stream::ReceiverStream<User>;
type ChatMessageStream = xrpc::stream::ReceiverStream<ChatMessage>;
```
