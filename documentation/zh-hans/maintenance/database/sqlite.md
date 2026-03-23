# SQLite 驱动实现细节

RBQ 的 SQLite 驱动是对嵌入式数据库的深度封装，通过 LIR 映射和高效的 FFI 交互实现零开销抽象。

## 1. 语法下放 (Lowering)

SQLite 的语法虽然简洁，但 RBQ 在下放时需要处理其特有的限制和特性：

### 1.1 类型亲和性 (Type Affinity) 处理
- **布尔下放**：映射为 `INTEGER`，其中 `1` 为真，`0` 为假。
- **日期时间**：下放为 ISO8601 格式的 `TEXT` 或 Unix 时间戳的 `INTEGER`，取决于模型注解配置。
- **UUID**：下放为 16 字节的 `BLOB` 以优化存储和索引速度。

### 1.2 算子下放映射
- **upsert 逻辑**：下放为 SQLite 3.24+ 支持的 `INSERT ... ON CONFLICT (...) DO UPDATE SET ...`。
- **分页下放**：标准 `LIMIT ... OFFSET ...`。
- **JSON 操作**：RBQ 的自由对象映射为 `TEXT` 字段，查询算子下放为 `json_extract()` 等内置函数。

### 1.3 自动迁移下放
由于 SQLite 对 `ALTER TABLE` 的支持有限，RBQ 的 Schema 迁移引擎在下放修改操作时，会自动生成“创建临时表 -> 拷贝数据 -> 重命名”的指令流。

## 2. 协议兼容性 (Protocol Compatibility)

SQLite 不使用网络协议，其“协议”体现为对 C API 的调用序列和内存管理。

### 2.1 FFI 绑定与内存安全
RBQ 通过 `libsqlite3-sys` 与 C 库交互：
- **Prepared Statements**：使用 `sqlite3_prepare_v2` 预编译 LIR 生成的 SQL。
- **参数绑定**：逻辑 `Value` 通过 `sqlite3_bind_*` 系列函数安全地绑定到语句句柄，避免 SQL 注入。
- **结果提取**：使用 `sqlite3_step` 迭代行，并通过 `sqlite3_column_*` 获取二进制或文本数据。

### 2.2 线程模型与并发
- **Serialized 模式**：驱动默认配置 SQLite 为 `SQLITE_CONFIG_SERIALIZED`，确保在多线程的异步运行时（如 Tokio）中，同一个连接句柄的并发调用也是安全的。
- **WAL 模式**：连接初始化时自动执行 `PRAGMA journal_mode=WAL`，大幅提升读写并发性能。

### 2.3 错误映射
- **Result Codes**：将 SQLite 的整数错误码（如 `19` SQLITE_CONSTRAINT）通过 `sqlite3_extended_errcode` 获取详细码，并映射为统一的 `RBQError`。

## 3. 性能优化特性
-   **内存数据库**：支持 `:memory:` 或 `file::memory:?cache=shared`，实现极速测试。
-   **共享缓存**：支持 `sqlite3_enable_shared_cache`，在多连接场景下减少内存占用。
-   **内建函数扩展**：RBQ 在连接启动时会自动注册一些常用的 Rust 实现函数到 SQLite（如高性能的正则引擎），以弥补其原生功能的不足。
