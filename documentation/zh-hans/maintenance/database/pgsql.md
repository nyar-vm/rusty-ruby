# PostgreSQL 驱动实现细节

RBQ 的 PostgreSQL 驱动基于原生的二进制 V3 协议实现，提供了从 RBQ IR 到 PostgreSQL 高级特性的深度映射。

## 1. 语法下放 (Lowering)

RBQ 的三层 IR 架构在 PostgreSQL 驱动中执行以下特定的下放逻辑：

### 1.1 标识符与参数化
- **转义规则**：所有表名和列名在 LIR 阶段被包裹在双引号 (`"`) 中，以支持大小写敏感和特殊字符标识符。
- **Schema 压平机制**：RBQ 对 PostgreSQL 的映射通过将物理 Schema 映射为逻辑 `namespace` 来实现压平。在驱动下放时，表名会根据该 `namespace` 绑定的 Schema 信息进行物理定位，从而在 RBQ 逻辑层消除 `database.schema.table` 的多级路径。
- **参数占位符**：RBQ 逻辑算子中的参数被下放为 PostgreSQL 特有的序号占位符（如 `$1`, `$2`, ...），而非传统的 `?`。

### 1.2 类型下放映射
- **UUID**：直接映射为 PostgreSQL 原生的 `uuid` 类型，而非字符串。
- **JSON/JSONB**：RBQ 中的自由对象和 `json` 类型默认下放为 `jsonb`，以利用其高效的索引和查询性能。
- **数组 (List)**：`list<T>` 被下放为 PostgreSQL 的 `ARRAY` 类型。

### 1.3 算子转换
- **字符串操作**：`contains` 算子下放为 `ILIKE`（不区分大小写）或 `~~*`。
- **正则匹配**：RBQ 正则注解下放为 `~` (POSIX 正则) 算子。
- **冲突处理**：RBQ 的 upsert 逻辑下放为 `INSERT ... ON CONFLICT (...) DO UPDATE SET ...`。

## 2. 协议兼容性 (Protocol Compatibility)

RBQ 遵循 PostgreSQL 的消息驱动模型，并针对二进制传输进行了优化。

### 2.1 扩展查询协议 (Extended Query)
为了实现高性能和安全性，RBQ 完整实现了 PostgreSQL 的扩展查询流程：
1.  **Parse**：将 LIR 生成的 SQL 语句发送给后端。RBQ 会根据 SQL 内容生成唯一的命名语句名以实现重用。
2.  **Bind**：RBQ 将参数从逻辑 `Value` 类型转换为 PostgreSQL 二进制格式。
    -   例如：`i32` 转换为 4 字节大端整数。
    -   `uuid` 转换为 16 字节原始二进制。
3.  **Describe**：获取结果集的元数据（列名、OID 类型等）。RBQ 利用这些信息在运行时进行类型安全校验。
4.  **Execute**：执行查询。
5.  **Sync**：强制刷新并确保所有操作在后端执行完毕。

### 2.2 二进制数据格式 (Binary Format)
RBQ 默认请求所有列以二进制格式返回（Format Code = 1）。这避免了文本解析的开销，尤其是在处理 `float`, `numeric` 和 `timestamp` 类型时。

### 2.3 状态机与错误处理
RBQ 解析 PostgreSQL 的 `ErrorResponse` 报文：
-   **字段提取**：提取 `'M'` (消息)、`'C'` (错误码/SQLSTATE) 和 `'D'` (详细信息)。
-   **统一映射**：将 PostgreSQL 的丰富错误信息映射为 `RBQError`。例如，`23505` 被精确映射为 `RBQError::UniqueViolation`。

## 3. 连接与安全
-   **启动消息 (Startup)**：在连接建立后发送 `StartupMessage`，包含版本信息、用户名、数据库名。
-   **TLS 支持**：内置 `rustls` 适配，支持 PostgreSQL 的 SSL 请求确认流程（发送 80877103 码）。
