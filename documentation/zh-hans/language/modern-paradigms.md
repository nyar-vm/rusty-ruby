# 现代数据范式：JSON 与向量

RBQ 并不局限于传统的行式关系数据。它完美支持非结构化文档模式以及当下火热的 AI 向量搜索。

## 1. 文档模式 (Document Pattern)

在 RBQ 中，没有标记 `@table` 的类被称为**自由对象**。

```rbq
class Meta {
    tags: list<utf8>;
    attributes: json;
}

@table(name = "items")
class Item {
    @key id: uuid;
    # 存储为 JSONB
    extra: Meta;
}
```

### 1.1 JSON 字段查询

你可以直接对 JSON 内部的字段进行过滤。

```rbq
items.filter { $extra.tags.contains("sale") }
```

## 2. AI 向量搜索 (@vector)

RBQ 原生支持向量类型，可以轻松集成到 RAG (检索增强生成) 工作流中。

```rbq
class Article {
    @key id: uuid;
    content: utf8;
    
    # 定义一个 1536 维的向量字段（如 OpenAI embedding 格式）
    @vector(index = "hnsw", metric = "cosine")
    embedding: vector<f32, 1536>;
}
```

### 2.1 相似度检索

使用 `.near()` 算子进行语义搜索。

```rbq
articles.near { $embedding, target_vector }
        .limit(5)
        .list();
```

## 3. 自动 Embedding (@embedding)

你甚至可以配置字段自动生成向量。

```rbq
class Product {
    name: utf8;
    
    @embedding(source = "name", model = "text-embedding-3")
    name_vec: vector<f32, 1536>;
}
```

**效果**：当 `name` 更新时，RBQ 会自动调用指定的模型生成新的向量并保存。

## 4. 分析型列存 (@columnar)

对于 OLAP（分析型）需求，指导编译器选择适合聚合查询的存储格式。

```rbq
@table(name = "sales_metrics")
@columnar
class Sale {
    @key id: u64;
    amount: d128;
    @audit(created_at)
    time: date_time;
}
```

**效果**: 编译器会优先选择支持列式存储的引擎或扩展（如 ClickHouse, Citus Columnar），显著提升聚合计算性能。

## 5. 大数据生命周期 (@partition, @ttl)

针对海量流水数据，RBQ 提供了原生的分区和清理支持。

```rbq
@table(name = "user_logs")
@partition(by = "range", field = "created_at", interval = "1 month")
@ttl(expire = "90d")
class UserLog {
    @key id: uuid;
    action: utf8;
    created_at: date_time;
}
```

*   **@partition**: 物理层自动创建按月分区的表。
*   **@ttl**: 自动删除 90 天前的数据，由数据库或引擎底层触发。

## 6. AI 特征工程 (@feature)

```rbq
struct UserProfile {
    @feature
    age: i32;
    @feature
    is_premium: bool;
}
```

**效果**: 标记为特征字段，方便 AI 系统自动提取特征用于模型训练或实时推理。

---

**下一节**：当内置功能不够用时，我们将学习如何利用 WASI (WebAssembly) 组件模型来无限扩展数据库的能力。
