# 工程增强：审计与版本控制

在企业级应用中，数据的可追溯性和一致性至关重要。RBQ 通过一组内置的“黑魔法”注解，自动化了这些繁琐的任务。

## 1. 自动化审计 (@audit)

如果你希望记录每一行数据的创建和修改时间，只需添加一个注解。

```rbq
@audit
@table(name = "orders")
class Order {
    @key id: uuid;
    total: d128;
}
```

**效果**：编译器会自动在物理表中注入 `created_at` 和 `updated_at` 字段，并在每次写入或更新时自动维护它们的值。

## 2. 逻辑删除 (@soft_delete)

直接从数据库物理删除数据通常是危险的。`@soft_delete` 让你的实体具备“逻辑删除”能力。

```rbq
@soft_delete
class Product {
    @key id: uuid;
    name: utf8;
}
```

**效果**：
*   **写入**：注入 `deleted_at` 字段。
*   **删除**：执行 `DELETE` 操作时，实际会转换为将 `deleted_at` 设置为当前时间。
*   **查询**：所有的查询算子（如 `filter`, `list`）都会自动忽略已删除的数据，除非你显式声明 `.with_deleted()`。

## 3. 乐观锁与版本控制 (@version)

为了防止并发更新导致的数据丢失，可以使用 `@version`。

```rbq
class Account {
    @key id: uuid;
    balance: d128;
    
    @version
    ver: i32;
}
```

**效果**：每次更新时，RBQ 会自动检查版本号是否一致，并将其加 1。如果版本冲突，更新将失败。

## 4. 运行时生命周期钩子 (Lifecycle Hooks)

除了编译器的自动化增强，你还可以定义在特定生命周期阶段触发的逻辑钩子。

```rbq
@before_save("encrypt_password")
@after_save("send_welcome_email")
@before_delete("check_dependencies")
class User {
    @key id: uuid;
    username: utf8;
    password_hash: utf8;
}
```

*   **@before_save**: 在数据保存到数据库之前触发（适用于加密、校验）。
*   **@after_save**: 在数据成功保存之后触发（适用于发送通知、同步缓存）。
*   **@before_delete**: 在执行删除操作之前触发（适用于关联检查）。

## 5. 数据库迁移与版本化 (Migrations)

虽然注解定义了“模型应该是什么样子”，但将这些变更应用到物理数据库则需要 CLI 工具的配合。

*   **模型变更**：你在 `.rbq` 文件中添加字段或注解。
*   **物理应用**：使用 `rbq migrate` 命令将模型差异演算并应用到数据库。

> 详细的 CLI 操作请参考：**[数据库迁移指南](../guide/commands/migration.md)**。

---

## 6. 小结
*   `@audit`：记住谁在什么时候做了什么。
*   `@soft_delete`：给数据一个“后悔药”。
*   `@version`：确保并发更新的安全性。

**下一节**：工程实践告一段落，我们将进入高级篇，学习如何编写编译期内联的 `micro` 函数。
