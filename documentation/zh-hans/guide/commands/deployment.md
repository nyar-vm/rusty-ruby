# 生产环境部署 (Deployment)

将 RBQ 应用于生产环境时，需要考虑流水线集成、机密管理以及零停机更新。

## 1. CI/CD 流水线集成

### 1.1 自动化检查
在 GitHub Actions 或 GitLab CI 的代码合并阶段，建议添加：
```yaml
# 检查模型语法和一致性
- name: RBQ Check
  run: rbq check
```

### 1.2 自动迁移
在部署阶段，应在应用启动前执行迁移：
```yaml
# 执行数据库迁移
- name: Run Migrations
  run: rbq migration up
  env:
    DATABASE_URL: ${{ secrets.DATABASE_URL }}
```

## 2. 机密管理 (Secrets)

**永远不要将数据库密码硬编码在 `.rbq` 文件中。**

RBQ 推荐使用环境变量或 `.env` 文件配合外部配置文件（如 `rbq.toml`）进行管理：

```toml
# rbq.toml 示例
[databases.main]
url = "${DATABASE_URL}" # 引用环境变量
```

## 3. 零停机迁移 (Zero-Downtime)

对于高可用应用，RBQ 建议遵循 "扩展-收缩" 模式进行 DDL 变更：

1.  **向后兼容变更**：先添加新列或新表（RBQ 迁移通常是增量的）。
2.  **代码更新**：部署新版本的 Rust 应用，使其开始使用新字段，同时保持对旧字段的兼容读写。
3.  **清理旧字段**：在确认新版本运行稳定后，再创建一个迁移来删除不再使用的旧字段。

## 4. 驱动性能调优

在生产环境的配置文件中，建议开启连接池与监控：

```toml
# rbq.toml 生产环境配置
[databases.main.pool]
max_connections = 20
min_connections = 5
idle_timeout = "30m"

[databases.main.logging]
slow_query_threshold = "200ms"
level = "warn"
```

---

## 5. 常见问题排查

- **连接失败**：检查安全组/防火墙设置，确保物理网络连通。
- **迁移死锁**：确保没有长事务在迁移执行期间持有大表的锁。
- **版本冲突**：如果多人同时修改 Schema，务必在合并代码后重新运行 `rbq migrate diff`。
