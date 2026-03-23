# 投影与转换

有时候你并不需要表中的所有字段（比如你只想拿用户的 ID 和姓名，而不想拿巨大的个人简介字段）。在 RBQ 中，我们使用 `map` 算子来实现数据的**投影 (Projection)** 和**转换 (Transformation)**。

## 1. 基础投影

`map` 算子允许你定义返回对象的结构。

```rbq
users.filter { $is_active }
     .map {
         id = $id
         name = $username
     }
     .list();
```

在这个例子中，返回的结果集将只包含 `id` 和 `name` 两个字段，而不是完整的 `User` 对象。

## 2. 计算与表达式

你可以在 `map` 闭包中执行简单的计算和字符串拼接。

```rbq
users.map {
    uid = $id
    full_name = $first_name + " " + $last_name
    can_vote = $age >= 18
}
```

## 3. 构造匿名对象

`map` 甚至可以生成嵌套的匿名结构。

```rbq
products.map {
    sku = $code
    info = {
        title = $name
        price_tag = "￥" + $price.to_utf8()
    }
}
```

## 4. 类型安全

由于 RBQ 是强类型的，编译器会检查 `map` 中的字段是否存在。如果你尝试访问 `$non_existent_field`，编译器在编译期就会报错。

---

**小结**：
*   `map` 决定了“查出什么”。
*   它不仅能选字段，还能重命名字段和计算新值。
*   减少网络传输带宽，只拿必要的数据。

**下一节**：我们将学习如何对数据进行分组和汇总，掌握 RBQ 强大的聚合分析能力。
