# 备份与恢复 (Backup & Restore)

数据安全是 RBQ 的核心关注点。RBQ CLI 提供了统一的接口，屏蔽了不同引擎备份工具（如 `pg_dump`, `mysqldump`）的参数差异。

## 1. 逻辑备份 (Logical Backup)

逻辑备份将数据导出为 RBQ 内部的 LIR 序列化格式或标准 SQL。这种方式跨版本兼容性好，适合中小型数据库。

### 1.1 执行备份

```bash
rbq backup --output ./backups/20240520.rbqdump
```

### 1.2 导出为压缩格式

```bash
rbq backup --compress gzip --output ./backups/full.sql.gz
```

## 2. 物理备份 (Physical Backup)

对于大型数据库（如 TB 级 PostgreSQL），RBQ 驱动层支持调用原生的快照或物理复制工具。

```bash
# 需要目标引擎支持物理备份插件
rbq backup --type physical --target s3://my-bucket/db-backups
```

## 3. 数据恢复 (Restore)

### 3.1 从文件恢复

```bash
rbq restore --from ./backups/20240520.rbqdump
```

### 3.2 恢复前的安全性检查

RBQ 在执行 `restore` 前会默认进行 Schema 校验。如果备份文件的 Schema 与当前代码中的 `.rbq` 模型不匹配，系统会提示警告。

```bash
# 忽略 Schema 匹配校验（强制恢复）
rbq restore --from ./dump.rbq --force
```

## 4. 自动备份任务 (Scheduled Backups)

虽然 RBQ CLI 自身不直接运行 Cron 任务，但它提供了易于集成的退出码和日志格式，方便与系统级调度工具集成：

```bash
# Linux Crontab 示例：每天凌晨 2 点备份
0 2 * * * /usr/local/bin/rbq backup --output /data/backup/daily.rbqdump >> /var/log/rbq-backup.log 2>&1
```

***

## 5. 跨引擎数据迁移

得益于 RBQ 的统一 IR，你可以利用 `backup` 和 `restore` 实现低成本的数据库换库：

- 场景：从 SQLite 迁移到 PostgreSQL。
- 操作：
  1. 连接 SQLite，执行 `rbq backup` 生成统一格式 dump。
  2. 修改 `database` 配置指向 PostgreSQL。
  3. 执行 `rbq migrate up` 初始化表结构。
  4. 执行 `rbq restore` 将数据导入。

## 6. 配置文件选项

`rbq.toml` 配置文件中与备份/恢复相关的选项：

### \[databases] 部分

- `url`: 数据库连接字符串，格式根据数据库类型而定
- `schema`: 可选的数据库架构名称，如 PostgreSQL 的 "public"

### \[databases.<name>.pool] 部分（生产环境推荐）

- `max_connections`: 最大连接数
- `min_connections`: 最小连接数
- `idle_timeout`: 空闲连接超时时间

