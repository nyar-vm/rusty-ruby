# 架构拉取 (Pull / Introspection)

`rbq pull` 命令用于从现有的数据库实例中提取 Schema 结构，并自动生成对应的 `.rbq` 模型文件。这是将现有项目迁移到 RBQ 的最快方式。

## 常用命令

### 从环境变量指定的数据库拉取
```bash
rbq pull --output ./models
```

### 指定数据库连接字符串
```bash
rbq pull --database "postgres://user:pass@localhost:5432/dbname" --output ./models
```

## 工作原理

1.  **元数据查询**：连接目标数据库，查询系统表（如 `information_schema`）。
2.  **类型映射**：将数据库原生类型（如 `varchar`, `bigint`）映射为 RBQ 类型（如 `String`, `i64`）。
3.  **关系识别**：识别外键约束并转换为 RBQ 的关联关系定义。
4.  **模型生成**：根据拉取到的信息生成 `.rbq` 文件。

## 配置文件选项

`rbq.toml` 配置文件中与拉取相关的选项：

### [project] 部分
- `source_dir`: RBQ 源文件目录，包含所有 `.rbq` 文件，默认为 "models"

### [databases] 部分
- `url`: 数据库连接字符串，格式根据数据库类型而定
- `schema`: 可选的数据库架构名称，如 PostgreSQL 的 "public"
