# ORM 概念

## 什么是 ORM

ORM (Object-Relational Mapping) 是一种编程技术，用于在面向对象编程语言中，将关系型数据库中的数据与对象模型之间建立映射关系。它允许开发者使用面向对象的方式操作数据库，而不需要直接编写 SQL 语句。

## ORM 的优势

1. **简化开发**：开发者可以使用熟悉的面向对象编程范式，而不需要学习 SQL 语法。
2. **提高可维护性**：通过对象模型，代码更加清晰、易于理解和维护。
3. **减少错误**：避免手动编写 SQL 语句带来的语法错误和安全问题（如 SQL 注入）。
4. **数据库无关性**：通过 ORM 抽象，应用程序可以在不同的数据库系统之间切换，而不需要修改业务逻辑代码。

## RBQ 中的 ORM 实现

RBQ 内置了自研的 ORM 系统，它具有以下特点：

1. **声明式建模**：使用 `.rbq` 文件定义数据模型，语法简洁明了。
2. **编译期代码生成**：在编译期生成最优的 Rust 代码，无运行时反射，性能优异。
3. **类型安全**：所有数据库操作都是类型安全的，避免运行时错误。
4. **查询构建器**：提供类型安全的查询构建器，支持链式调用。
5. **事务支持**：内置事务管理，确保数据一致性。
6. **关系映射**：支持一对一、一对多、多对多等关系映射。

## 示例

```rbq
model User {
    id: i64 = 0;
    @unique username: string;
    email: string;
    created_at: datetime = now();
    
    // 关系
    posts: Post[];
}

model Post {
    id: i64 = 0;
    title: string;
    content: string;
    author_id: i64;
    
    // 外键关联
    @belongs_to author: User;
    created_at: datetime = now();
}
```

生成的 Rust 代码可以这样使用：

```rust
// 数据库操作
let db = user::db::connect().await.unwrap();

// 创建用户
let user = User {
    id: 0,
    username: "john",
    email: "john@example.com",
    created_at: chrono::Utc::now(),
    posts: None,
};
let saved_user = user.insert(&db).await.unwrap();

// 查询用户
let found_user = User::find_by_id(&db, saved_user.id).await.unwrap();

// 使用查询构建器
let users = User::query()
    .filter(|u| u.username.like("%john%"))
    .order_by(|u| u.created_at.desc())
    .fetch(&db).await.unwrap();
```