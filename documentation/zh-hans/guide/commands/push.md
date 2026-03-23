# 架构推送 (Push)

`rbq push` 命令用于将 RBQ 模型定义推送到数据库，自动创建或更新数据库表结构。这是将模型定义应用到数据库的主要方式。

## 常用命令

### 推送到环境变量指定的数据库
```bash
rbq push
```

### 指定数据库连接字符串
```bash
rbq push --database "postgres://user:pass@localhost:5432/dbname"
```

### 仅生成迁移文件，不执行
```bash
rbq push --dry-run
```

## 工作原理

1.  **模型解析**：解析项目中的 `.rbq` 文件，构建内存中的 Schema 模型。
2.  **差异比较**：与目标数据库的现有结构进行比较，识别需要创建、修改或删除的对象。
3.  **迁移生成**：根据差异生成对应的 DDL 语句。
4.  **执行应用**：执行生成的 DDL 语句，将变更应用到数据库。

## 配置文件选项

`rbq.toml` 配置文件中与推送相关的选项：

### [project] 部分
- `source_dir`: RBQ 源文件目录，包含所有 `.rbq` 文件，默认为 "models"

### [databases] 部分
- `url`: 数据库连接字符串，格式根据数据库类型而定
- `schema`: 可选的数据库架构名称，如 PostgreSQL 的 "public"