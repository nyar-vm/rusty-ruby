# 逻辑封装：Micro 函数

在传统的数据库开发中，逻辑复用通常通过“存储过程”或“视图”来实现。但这会带来维护困难、版本控制失效等问题。RBQ 引入了 **Micro 函数**，它是一种在**编译期内联 (Inline)** 的逻辑单元。

## 1. 什么是 Micro 函数？

`micro` 函数是 RBQ 语言中的一级公民。它的特点是：
*   **物理无关**：不依赖具体的数据库特性。
*   **零开销**：在 HIR 阶段直接展开，不增加运行时的 SQL 负担。
*   **类型安全**：输入输出受 RBQ 类型系统保护。

## 2. 定义与使用

```rbq
# 定义一个判断是否为成人的逻辑
micro is_adult(age: u32?) -> bool {
    age >= 18
}

# 在查询中使用
users.filter { is_adult($age) }
```

当编译器看到上面的查询时，它会自动将其展开为：
`users.filter { $age >= 18 }`

## 3. 复杂逻辑封装

你可以在 `micro` 函数中包含复杂的布尔逻辑。

```rbq
micro can_publish(user: User, post: Post) -> bool {
    user.is_active && !user.is_banned && post.status == PostStatus.Draft
}

# 优雅的查询
users.with { $posts }
     .filter { can_publish($, $posts.first()) }
```

## 4. 为什么叫 "Micro"？

因为它被设计为处理**微小、纯粹的逻辑片段**。它不应该包含副作用，也不应该直接操作数据库连接。它只是对查询条件的语义化包装。

---

**小结**：
*   `micro` 是编译器的语法糖。
*   它让你的查询代码更具可读性（语义化）。
*   告别散落在各处的硬编码逻辑。

**下一节**：我们将学习 RBQ 是如何拥抱未来的——JSON 文档模式与 AI 向量搜索集成。
