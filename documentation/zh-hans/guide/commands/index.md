# RBQ CLI 工具指南

RBQ 提供了一个强大的命令行界面 (CLI) 工具，用于管理数据库的生命周期，包括初始化、迁移、备份、恢复以及生产环境部署。

## 核心命令概览

| 命令 | 说明 | 适用场景 |
| :--- | :--- | :--- |
| **[rbq check](check.md)** | 检查 .rbq 文件语法与语义错误 | 开发阶段校验 |
| **[rbq compile](compile.md)** | 将 .rbq 文件编译为目标代码 (Rust/TS) | 代码生成 |
| **[rbq generate](generate.md)** | 生成查询或文档 | 辅助开发 |
| **[rbq migration](migration.md)** | 数据库迁移管理 (create, up, down, status) | 结构变更同步 |
| **[rbq pull](pull.md)** | 从现有数据库拉取 Schema | 逆向工程 |
| **[rbq push](push.md)** | 将模型定义推送到数据库 | 结构变更应用 |
| **[rbq backup](backup.md)** | 数据库物理或逻辑备份 | 数据安全保障 |
| `rbq restore` | 从备份中恢复数据 | 灾难恢复 |

---

## 快速导航

1. **[语法检查 (Check)](check.md)**：验证模型定义的合法性。
2. **[代码编译 (Compile)](compile.md)**：将模型转换为强类型代码。
3. **[数据库迁移 (Migration)](migration.md)**：如何安全地管理 Schema 变更。
4. **[架构拉取 (Pull)](pull.md)**：从现有数据库逆向生成模型。
5. **[架构推送 (Push)](push.md)**：将模型定义应用到数据库。
6. **[备份与恢复 (Backup & Restore)](backup.md)**：跨引擎的数据保护方案。
7. **[生产环境部署 (Deployment)](deployment.md)**：CI/CD 集成与最佳实践。

---

## 相关资源

- **[概念指南](../concepts/index.md)**：了解 ORM、RPC、xRPC 等核心概念。
- **[RBQ 语言指南](../../language/index.md)**：学习 RBQ DSL 的语法和特性。
- **[架构指南](../architecture.md)**：深入了解 RBQ 的架构设计。
