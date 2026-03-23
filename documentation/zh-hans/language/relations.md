# 关系、外键与预加载

在关系型数据库中，数据通过主外键连接。RBQ 提供了两种方式来表达这种联系：**物理外键**与**逻辑关联**。

## 1. 物理外键 (&T)

如果你希望在数据库中创建一个真实的 `FOREIGN KEY` 约束，使用 `&T` 语法。

```rbq
@table(name = "posts")
class Post {
    @key id: uuid;
    title: utf8;
    
    # 物理外键：在数据库中生成 author_id 列，并添加约束
    author: &User; 
}
```

*   **&User**：代表这是一个指向 `User` 实体的主键引用。
*   **物理存储**：默认存储为被引用表主键的类型（如 `uuid` 或 `u32`）。

## 2. 逻辑关联 (@relation)

有时候你不需要物理约束（为了分布式数据库的兼容性或性能），或者你想定义反向关系。这时使用 `@relation`。

```rbq
@table(name = "users")
class User {
    @key id: uuid;
    
    # 逻辑关联：一对多关系
    # 告诉编译器：去 posts 表里找 author 字段等于当前 id 的记录
    @relation(field = "id", target = "Post.author")
    posts: list<Post>;
}
```

## 3. 预加载 (Eager Loading: with)

这是解决 N+1 查询问题的利器。默认情况下，RBQ 不会加载关联数据。如果你需要它们，使用 `with` 算子。

```rbq
# 查询用户，并同时抓取他们的所有帖子
users.with { $posts }
     .list();
```

### 3.1 嵌套加载

你可以一直链式加载下去。

```rbq
users.with { 
    $posts.with { $comments } 
}
.list();
```

## 4. 自动 JOIN 优化

RBQ 的编译器非常聪明。当你编写 `users.filter { $posts.any { $title.contains("RBQ") } }` 时，它会自动在 MIR 层生成高效的 `EXISTS` 或 `JOIN` 语句，而不需要你手动去写复杂的 JOIN 逻辑。

---

**小结**：
*   `&T` 创建强约束。
*   `@relation` 定义逻辑上的联系。
*   `with` 解决 N+1，让数据加载更高效。

**下一节**：我们将正式开启 RBQ 的查询之旅，学习如何使用强大的函数式 DSL 来获取和过滤数据。
