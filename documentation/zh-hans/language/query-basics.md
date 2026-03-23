# 数据流思想与基础查询

RBQ 的查询语法与传统的 SQL 截然不同。它采用了一种类似于 Rust 迭代器或 Java Stream 的**函数式管道 (Functional Pipeline)** 风格。

## 1. 核心理念：数据流 (Data Flow)

在 RBQ 中，查询被视为一个从“数据源”开始，经过一系列“转换算子”，最后达到“结果集”的过程。

```rbq
# 一个典型的 RBQ 查询
users.filter { $age > 18 }
     .sort { $created_at.desc() }
     .limit(10)
     .list();
```

*   **users**：数据源（定义的实体名）。
*   **filter / sort / limit**：转换算子（管道函数）。
*   **list()**：终止算子（触发执行并返回列表）。

## 2. 基础过滤：filter

`filter` 算子接收一个闭包，返回为 `true` 的行将被保留。

### 2.1 魔法变量 `$`

在闭包内部，`$` 代表“当前行对象”。
*   `$age` 是 `$.age` 的简写。
*   `$profile.name` 用于访问嵌套对象的字段。

### 2.2 逻辑运算符

*   **比较**：`==`, `!=`, `>`, `<`, `>=`, `<=`.
*   **组合**：`&&` (与), `||` (或), `!` (非).
*   **集合**：`in` (在列表中), `contains` (包含字符串/子项).

```rbq
# 示例：查找名字叫 Alice 且年龄在 18-25 之间的用户
users.filter { $name == "Alice" && $age >= 18 && $age <= 25 }
```

## 3. 排序与分页

*   **sort**：使用 `.asc()` (升序) 或 `.desc()` (降序)。
*   **limit**：限制返回数量。
*   **offset**：跳过前 N 条记录。

```rbq
products.sort { $price.asc(), $id.desc() }
        .limit(20)
        .offset(40)
        .list();
```

## 4. 获取单条数据

除了 `.list()`，你还可以使用以下终止算子：
*   **first()**：返回第一条记录，如果没有则报错。
*   **first_opt()**：返回第一条记录的 `Option`。
*   **count()**：仅返回符合条件的数量。

## 5. 闭包与语法糖 (Closures & Syntax Sugar)

RBQ 引入了 **尾随闭包 (Trailing Closure)** 语法，使得 DSL 的表达更加简洁。

*   **尾随闭包**: 当函数的最后一个参数是闭包时，可以将其写在圆括号之外。
*   **匿名参数**: 在闭包内部，`$` 等价于第一个参数。多参数可以使用 `$1`, `$2` 等。
*   **属性简写**: `$field` 是 `$.field` 的语法糖。

```rbq
# 完整形式
users.filter(func($) { $.age > 18 })

# 尾随闭包 + 属性简写 (推荐)
users.filter { $age > 18 }
```

## 6. 数据变更 (Mutation)

除了查询，DSL 还支持类型安全的数据变更操作。

### 6.1 插入 (Insert)
```rbq
users.insert({ username: "alice", email: "alice@example.com" })

# 批量插入
users.insert([
  { username: "bob", email: "bob@example.com" },
  { username: "charlie", email: "charlie@example.com" }
])
```

### 6.2 更新 (Update)
```rbq
users.filter { $id == 1 }
     .update { is_active = false }
```

### 6.3 删除 (Delete)
```rbq
users.filter { $is_deleted }.delete()
```

---

**小结**：
*   查询像流一样从上往下流动。
*   使用 `filter` 进行条件筛选。
*   记住快捷键 `$`：它总是指向当前正在处理的那行数据。

**下一节**：我们将学习如何使用 `map` 算子对数据进行“整形”，只取出我们需要的字段。
