# RBQ 语言指南 (RBQ Language Guide)

欢迎来到 RBQ 语言的学习之旅。本指南将带你从零开始，逐步掌握这门专为现代 Rust 后端设计的声明式建模与查询语言。

我们建议你按照以下顺序循序渐进地阅读：

## 第一阶段：核心基石 (The Foundation)
理解 RBQ 的本质，掌握最基础的数据表达方式。

1.  **[设计哲学](philosophy.md)**：为什么我们需要 RBQ？理解“赛博商务”美学与三层 IR 架构。
2.  **[快速上手：基础语法](basics.md)**：注释、基本类型以及如何定义你的第一个类。
3.  **[实体建模：@table 与主键](entity-modeling.md)**：理解实体表与自由对象的区别，掌握主键与索引。
4.  **[代数数据类型：Unions 与 ADT](unions.md)**：超越简单的联合体，学习带参数的 ADT 及其物理表示。
5.  **[关系、外键与预加载](relations.md)**：理解 `&T` (物理外键) 与 `@relation` (逻辑关联)，解决 N+1 问题。

## 第二阶段：函数式查询 (Functional Query DSL)
数据定义好了，现在学习如何高效地查询和转换它们。

6.  **[数据流思想与基础查询](query-basics.md)**：理解 `filter` 管道与基础比较操作。
7.  **[投影与转换](projection.md)**：使用 `map` 塑形数据，掌握 `$field` 简写语法。
8.  **[聚合与统计](aggregation.md)**：分组、计数、求和等高级数据分析。

## 第三阶段：工程组织与安全 (Engineering & Organization)
当项目变大、变复杂时，如何保持代码的可维护性与安全性。

9.  **[字段复用：Traits 与 Mixins](traits.md)**：使用 `trait` 和 `using` 减少模板代码，实现横切关注点。
10. **[多数据库混合与命名空间](namespaces.md)**：使用 `namespace` 组织大型项目，掌握 PostgreSQL Schema 压平规则。
11. **[安全与隔离：策略与租户](security.md)**：使用 `@policy` 实现行级安全，利用 `@tenant` 实现原生隔离。
12. **[工程增强：内置黑魔法](engineering.md)**：掌握 `@audit`, `@soft_delete`, `@version` 等工程化注解。

## 第四阶段：现代范式与扩展 (Modern Paradigms & Extensions)
探索 RBQ 的边界，处理非结构化数据与自定义逻辑。

13. **[逻辑封装：Micro 函数](micro-functions.md)**：定义编译期内联的纯逻辑算子。
14. **[现代数据范式：JSON 与向量](modern-paradigms.md)**：文档模式与 AI 向量搜索集成。
15. **[WASI UDF 扩展](udf-wasi.md)**：利用 WebAssembly 组件模型扩展数据库能力。

## 进阶：运维与底层机制
掌握了建模语言后，你可能需要深入了解如何管理数据库的生命周期或其运行机理。

- **[RBQ CLI 工具指南](../guide/commands/index.md)**：迁移、备份与生产部署。
  - **[维护指南](../maintenance/index.md)**：深入理解编译器 IR、驱动适配器与内部机制。
