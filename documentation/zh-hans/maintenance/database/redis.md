# Redis 驱动实现细节

RBQ 的 Redis 驱动不仅是 RESP 协议的简单封装，它还将复杂的逻辑查询下放为高效的 Redis 命令流。

## 1. 语法下放 (Lowering)

由于 Redis 是非关系型数据库，RBQ 的查询 DSL 需要进行深度的算子下放转换：

### 1.1 集合与键值下放
- **简单查询**：针对单一实体的 `filter { $id == "..." }` 被下放为 `GET` 或 `HGET` 命令。
- **批量查询**：`list()` 算子在 LIR 阶段被下放为 `SCAN`（用于集合遍历）或 `MGET`（用于已知 ID 列表）。

### 1.2 复杂算子与 Lua 下放
对于 Redis 原生不支持的复杂过滤或原子更新，RBQ 会将其下放为嵌入式 Lua 脚本：
- **复杂过滤**：当 `filter` 包含非索引字段的逻辑运算时，RBQ 生成 Lua 脚本在服务端进行过滤，减少网络传输。
- **原子更新**：`update` 逻辑被下放为 `EVAL` 命令包裹的 Lua 脚本，以确保 ACID 中的原子性。

### 1.3 索引下放模拟
如果模型中定义了 `@index`，RBQ 会自动管理辅助键（Secondary Indexes）：
- **唯一索引**：下放为额外的 `SET` 键。
- **范围索引**：下放为 `ZSET` (Sorted Set)，将过滤操作转换为 `ZRANGEBYSCORE`。

## 2. 协议兼容性 (Protocol Compatibility)

RBQ 实现了完整的 RESP (Redis Serialization Protocol) 解析器，并支持高性能交互模式。

### 2.1 RESP 编码与解码
RBQ 的 `Value` 类型与 RESP 类型存在严格映射：
- **Bulk Strings**：对应 RBQ 的 `string` 和 `binary`。
- **Integers**：对应 `i64`。
- **Arrays**：对应 `list<T>`。
- **Simple Errors**：直接解析并转换为 `RBQError::DatabaseError`。

### 2.2 流水线与批处理 (Pipelining)
RBQ 的执行引擎会自动优化连续的 Redis 操作：
- **Pipelining**：在没有数据依赖的情况下，将多个下放后的命令一次性打包发送，减少往返时间 (RTT)。
- **RESP 块解析**：解析器支持流式处理响应块，无需等待完整数组返回即可开始处理首个元素。

### 2.3 订阅模式 (Pub/Sub)兼容
通过 `DriverExtension`，RBQ 支持 Redis 的推送消息：
- **状态切换**：当进入订阅模式后，驱动的状态机自动切换到异步监听模式，处理以 `*3\r\n$7\r\nmessage` 开头的特殊报文。

## 3. 连接与拓扑支持
-   **自动重连**：驱动层实现了基于指数退避的自动重连机制。
-   **集群感知 (Preview)**：支持解析 `-MOVED` 错误并自动更新内部槽位映射表。
