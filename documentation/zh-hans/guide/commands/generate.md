# 代码与文档生成 (Generate)

`rbq generate` 命令用于基于已有的 Schema 生成辅助性的产物，如复杂的查询代码、API 文档等。

## 常用命令

### 生成查询代码
```bash
rbq generate queries --output ./src/queries
```

### 生成数据库文档
```bash
rbq generate docs --output ./docs/schema
```

## 生成内容

- **Queries**: 生成基于模型的强类型查询函数，减少手写 SQL 或 DSL 的工作量。
- **Docs**: 生成可读性强的 Markdown 或 HTML 文档，描述数据库表结构、字段含义及关联关系。

## 配置文件选项

`rbq.toml` 配置文件中与生成相关的选项：

### [project] 部分
- `source_dir`: RBQ 源文件目录，包含所有 `.rbq` 文件，默认为 "models"

### [databases] 部分
- `url`: 数据库连接字符串，格式根据数据库类型而定
- `schema`: 可选的数据库架构名称，如 PostgreSQL 的 "public"

### [generate] 部分
- `target`: 目标语言，支持 "rust" 或 "typescript"
- `output`: 生成代码的输出目录

### [generate.rust] 子部分
- `serde`: 是否为生成的类型添加 serde 序列化/反序列化支持，默认为 false
- `runtime`: 是否生成运行时支持代码，默认为 false
