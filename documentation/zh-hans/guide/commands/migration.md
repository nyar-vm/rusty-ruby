# 数据库迁移 (Migration)

在 RBQ 中，迁移不再是手写脆弱的 SQL 脚本，而是基于模型（Schema）的自动演算。

## 1. 核心流程

RBQ 的迁移遵循 **"声明即事实"** 的原则：

1.  **修改模型**：在 `.rbq` 文件中修改 `class`, `union` 或注解。
2.  **生成差异**：运行 `rbq migration create`，CLI 会对比当前 `.rbq` 模型与物理数据库的差异。
3.  **生成脚本**：自动生成带时间戳的迁移文件（包含 SQL 和 RBQ 内部状态）。
4.  **应用迁移**：运行 `rbq migration up` 执行变更。

## 2. 常用命令

### 2.1 创建新迁移
```bash
rbq migration create --name "add_user_email"
```
这会在 `migrations/` 目录下生成一个新的迁移文件夹。

### 2.2 执行迁移
```bash
rbq migration up
```
RBQ 会自动识别尚未应用的迁移，并在物理数据库中按顺序执行。

### 2.3 回滚迁移
```bash
rbq migration down
```
回滚最近的一次操作。**注意：回滚可能会导致数据丢失，请务必在生产环境谨慎操作。**

### 2.4 查看状态
```bash
rbq migration status
```
查看当前已应用和待应用的迁移状态。

## 3. 影子数据库 (Shadow Database)

为了确保迁移脚本的 100% 正确，RBQ 在生成 `diff` 时会要求连接一个**影子数据库**：
- 它是一个临时的、空白的数据库实例。
- RBQ 会将当前的 `.rbq` 模型先应用到影子数据库，确保没有任何语法或方言冲突，再生成最终的迁移脚本。

## 4. 迁移版本管理

RBQ 在目标数据库中维护一张 `_rbq_migrations` 表，记录：
- 迁移的版本号 (Timestamp)。
- 迁移名称。
- 应用时间。
- 校验和 (Checksum)，用于防止已应用的迁移脚本被篡改。

---

## 5. 最佳实践

- **原子性**：RBQ 尽可能在事务中执行 DDL（取决于目标引擎的支持情况，如 PostgreSQL 支持，而 MySQL 不支持部分 DDL 事务）。
- **版本控制**：务必将 `migrations/` 目录提交到 Git。
- **不可变性**：一旦迁移已部署到生产环境，**严禁**修改已生成的迁移脚本。如果需要修正，请创建一个新的 `diff`。

## 6. 配置文件选项

`rbq.toml` 配置文件中与迁移相关的选项：

### [databases] 部分
- `url`: 数据库连接字符串，格式根据数据库类型而定
- `schema`: 可选的数据库架构名称，如 PostgreSQL 的 "public"

### [databases.<name>.pool] 部分（生产环境推荐）
- `max_connections`: 最大连接数
- `min_connections`: 最小连接数
- `idle_timeout`: 空闲连接超时时间

### [databases.<name>.logging] 部分（生产环境推荐）
- `slow_query_threshold`: 慢查询阈值
- `level`: 日志级别
