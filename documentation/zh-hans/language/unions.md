# 代数数据类型：Unions 与 ADT

在 RBQ 中，`union` 不仅仅是一组常量，它还是功能强大的 **代数数据类型 (Algebraic Data Types, ADT)**，类似于 Rust 的枚举。

## 1. 简单联合体

最常见的场景是定义一组固定的状态。

```rbq
union OrderStatus {
    Pending;
    Paid;
    Shipped;
    Cancelled;
}
```

## 2. 带参数的 ADT

RBQ 的联合体成员可以携带数据，这在处理多态数据时非常有用。

```rbq
union PaymentMethod {
    CreditCard { number: utf8, holder: utf8 };
    Alipay { account: utf8 };
    Cash;
}
```

## 3. 物理表示 (@repr)

为了让数据库理解这些复杂的结构，RBQ 提供了多种映射方式。

### 3.1 字符串映射 (默认)

默认情况下，简单联合体会被映射为字符串。

### 3.2 数值映射 (@repr(u8/i32/...))

如果你希望节省空间，可以将其映射为数值。

```rbq
@repr(u8)
union Level {
    Low = 1;
    Medium = 2;
    High = 3;
}
```

### 3.3 带标签的映射 (@tag)

对于带参数的 ADT，RBQ 默认使用 JSON 格式存储。你可以通过 `@tag` 控制标签字段的名字。

```rbq
@tag("type")
union Message {
    Text { content: utf8 };
    Image { url: utf8, size: i32 };
}
# 存储示例: {"type": "Text", "content": "hello"}
```

## 4. 状态机流转 (@transition)

对于表示状态的联合体，RBQ 允许你定义合法的状态流转规则。这可以在运行期自动校验状态变更是否符合业务逻辑。

```rbq
union OrderStatus {
    @initial
    Pending;
    
    @transition(from = [Pending])
    Processing;
    
    @transition(from = [Processing])
    Completed;
    
    @transition(from = [Pending, Processing])
    Cancelled;
}
```

*   **@initial**: 标记初始状态。
*   **@transition(from = [...])**: 限制哪些状态可以流转到当前状态。

## 5. 模式匹配 (Preview)

在查询 DSL 中，你可以直接对联合体进行过滤和匹配。

```rbq
orders.filter { $status == OrderStatus.Paid }
```

---

**小结**：
*   简单联合体用于状态。
*   带参数的 ADT 用于复杂多态。
*   使用 `@repr` 或 `@tag` 控制物理存储细节。

**下一节**：我们将学习 RBQ 如何处理实体之间的复杂关系与外键，构建起完整的数据网格。
