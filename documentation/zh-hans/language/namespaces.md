# 多数据库混合与命名空间

在复杂的企业架构中，你的数据可能分布在不同的物理数据库中（例如：用户数据在 PostgreSQL，而日志数据在 ClickHouse）。RBQ 提供了 `namespace` 和 `@database` 来优雅地处理这种多源混合场景。

## 1. 为什么需要命名空间？

如果你只有一个数据库，你可以完全不使用 `namespace`。但当你面临以下场景时，它是不可或缺的：

*   **物理库隔离**：将不同的模型集映射到不同的物理数据库连接。
*   **大型项目组织**：避免数百个模型堆在一起导致的名字冲突。
*   **跨库关联**：在 A 库的模型中引用 B 库的模型。

## 2. 定义命名空间 (Namespace)

`namespace` 为模型提供了一个逻辑上的层级结构。

**原则：一文件一命名空间 (One File, One Namespace)**
在 RBQ 的工程实践中，原则上**一个 `.rbq` 文件只允许编写一个 `namespace`**。这有助于保持项目结构的清晰，并简化物理映射逻辑。

```rbq
# auth.rbq 文件
namespace auth;

@table(name = "users")
class User { ... } # 逻辑全名为 auth.User
```

## 3. 多数据库映射 (@database)

你可以使用 `@database` 将整个命名空间绑定到特定的物理数据库连接。在 RBQ 中，**`@database` 应当标注在 `namespace` 关键字之上**。

### 3.1 PostgreSQL 的特殊映射规则

对于 PostgreSQL 用户，RBQ 采用的是 **“Schema 压平至命名空间”** 的映射哲学：

*   **压平映射**：PostgreSQL 物理库中的每一个 **Schema**（如 `public`, `auth`, `inventory`）在 RBQ 中都被映射为一个独立的逻辑 `namespace`。
*   **一命名空间一绑定**：每一个 `namespace` 通过 `@database` 绑定到具体的物理库。RBQ 并不在 DSL 中保留 `database.schema.table` 的三级深度，而是通过这种压平机制，让所有表在逻辑层级上看起来是平铺在各个命名空间下的。
*   **设计初衷**：这种设计强制开发者将逻辑模块与物理 Schema 一一对应。在编写 RBQ 代码时，你只需关注 `auth.User` 或 `inventory.Product`，而底层的物理存储位置（在哪个库、哪个 Schema）则由绑定配置决定。

```rbq
# auth.rbq 文件
@database("main_pg_db") # 在配置中，此连接已指定 search_path 或 schema 为 'auth'
namespace auth;

@table(name = "users") # 逻辑名 auth.User，物理对应 main_pg_db.auth.users
class User { ... }

# inventory.rbq 文件
@database("main_pg_db") # 同样绑定到主库，但在配置中对应 'inventory' schema
namespace inventory;

@table(name = "products") # 逻辑名 inventory.Product，物理对应 main_pg_db.inventory.products
class Product { ... }
```

### 3.2 覆盖规则

虽然建议在命名空间级别统一配置，但如果某个特定的实体表需要存储在不同的物理位置，你仍然可以在 `@table` 上单独声明来覆盖全局配置。

```rbq
# business.rbq 文件
@database("postgres_main")
namespace business;

@table(name = "orders")
class Order { ... }

# 特殊情况：将归档表映射到外部存储
@database("s3_archive")
@table(name = "order_archives")
class OrderArchive { ... }
```

## 4. 传播与覆盖规则

*   **传播**：定义在 `namespace` 上的 `@database` 会自动应用到该命名空间下的所有 `@table`。
*   **覆盖**：如果某个类有特定的存储需求，可以在该类上单独声明 `@database` 来覆盖命名空间的配置。

## 5. 跨命名空间引用与导入

```rbq
namespace blog;
using auth.User; # 导入其他命名空间的实体

class Post {
    author: User; # 建立跨库/跨模块的逻辑联系
}
```

---

**小结**：
*   单数据库项目：**不需要**写 namespace。
*   多数据库项目：使用 `namespace` + `@database` 进行物理绑定。
*   使用 `using` 简化跨模块引用。

**下一节**：数据组织好了，现在让我们学习如何保护它们——RBQ 的行级安全策略与多租户隔离。
