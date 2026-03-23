# MySQL 驱动实现细节

RBQ 的 MySQL 驱动旨在提供高性能、安全的连接方案，通过严格的 LIR 映射和二进制协议确保执行效率。

## 1. 语法下放 (Lowering)

RBQ 在针对 MySQL 生成物理指令时，会处理其特定的语法特性：

### 1.1 标识符与转义
- **转义规则**：所有标识符（表名、列名）在下放时被包裹在反引号 (`` ` ``) 中。
- **分页下放**：RBQ 的 `limit` 和 `offset` 直接映射为 MySQL 的 `LIMIT offset, limit` 语法。

### 1.2 类型下放映射
- **布尔值**：RBQ 的 `bool` 类型被下放为 MySQL 的 `TINYINT(1)`，其中 `true` 为 1，`false` 为 0。
- **JSON**：RBQ 的 JSON 字段和自由对象映射为 MySQL 的 `JSON` 原生类型。
- **日期时间**：`date_time` 下放为 `DATETIME(6)`，以支持微秒精度。

### 1.3 算子转换
- **upsert 逻辑**：RBQ 的 upsert 操作被下放为 `INSERT ... ON DUPLICATE KEY UPDATE ...` 语句。
- **字符串操作**：`contains` 算子下放为 `LIKE CONCAT('%', ?, '%')`。
- **正则表达式**：下放为 MySQL 的 `REGEXP_LIKE` 函数。

## 2. 协议兼容性 (Protocol Compatibility)

RBQ 强制使用 MySQL 二进制协议 (Prepared Statements) 进行数据交互。

### 2.1 参数化查询流程
1.  **COM_STMT_PREPARE**：发送 SQL 模板到服务器。服务器返回一个 `Statement ID` 以及参数和列的元数据。
2.  **参数编码**：RBQ 将逻辑 `Value` 转换为 MySQL 的二进制报文格式。
    -   使用 **Null Bitmap** 标记空值参数。
    -   根据类型码（如 `MYSQL_TYPE_LONG`, `MYSQL_TYPE_STRING`）进行二进制编码。
3.  **COM_STMT_EXECUTE**：使用 `Statement ID` 和编码后的参数执行查询。

### 2.2 身份验证机制
RBQ 实现了 MySQL 的身份验证切换（Authentication Method Switch）逻辑，重点支持现代安全插件。

#### **Caching SHA2 Password (默认)**
-   **处理方式**：支持 RSA 公钥加密传输或在 SSL 环境下直接传输。
-   **哈希算法**：使用 SHA256 对密码进行迭代哈希处理，安全性远高于旧版的 `mysql_native_password`。

### 2.3 报文解析与状态机
RBQ 实现了一个轻量级的流式报文解析引擎：
-   **流式处理**：解析 4 字节的 MySQL 报文头（包含长度和序列号）。
-   **结果集解析**：在 `COM_STMT_EXECUTE` 后，按序解析 `Column Definition`、`EOF` 以及 `Binary Protocol Resultset Row`。
-   **错误映射**：解析 MySQL 的 `ERR_Packet`，将错误码（如 `1062` 唯一约束冲突）映射为 `RBQError`。

## 3. 连接与池化
-   **连接设置**：在握手阶段自动协商 `CLIENT_PROTOCOL_41` 和 `CLIENT_SECURE_CONNECTION` 等功能标志。
-   **字符集**：默认强制使用 `utf8mb4` 字符集，以支持完整的 Unicode。
