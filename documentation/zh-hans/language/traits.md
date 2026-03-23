# 字段复用：Traits 与 Mixins

在后端开发中，很多表都拥有重复的字段，比如 `created_at`, `updated_at`, `is_deleted` 等。手动为每个表添加这些字段不仅枯燥，还容易出错。RBQ 通过 `trait` 机制完美解决了这个问题。

## 1. 定义 Trait

`trait` 是一组字段定义的集合。它本身不生成物理表，只能被其他类 (Class) 引用。

```rbq
trait Timestamp {
    created_at: date_time;
    updated_at: date_time;
}

trait SoftDelete {
    is_deleted: bool = false;
    deleted_at: date_time?;
}
```

## 2. 使用 Trait (Mixins)

在类中使用 `using` 关键字将 trait 的字段“混入”到当前模型中。

```rbq
@table(name = "posts")
class Post {
    @key id: uuid;
    title: utf8;
    
    using Timestamp;
    using SoftDelete;
}
```

### 2.1 物理层面的效果

当编译器处理 `Post` 时，它会将 `Timestamp` 和 `SoftDelete` 中定义的字段全部展开。最终生成的物理表 `posts` 将包含 6 个字段。

## 3. 带有注解的 Trait

你甚至可以在 trait 中包含注解，这些注解也会被完整复用。

```rbq
trait Audit {
    @audit
    using Timestamp;
}

class User {
    @key id: uuid;
    using Audit;
}
```

## 4. Trait 的组合

Trait 可以互相嵌套，形成功能强大的逻辑块。

```rbq
trait StandardMetadata {
    using Timestamp;
    using SoftDelete;
    created_by: uuid;
}
```

---

**小结**：
*   `trait` 用于定义可复用的字段集。
*   使用 `using` 进行代码混入。
*   这不仅是代码复用，更是业务规范的统一。

**下一节**：我们将学习如何组织大型项目，以及 RBQ 独特的命名空间与多数据库绑定机制。
