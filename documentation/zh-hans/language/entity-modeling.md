# 实体建模：@table 与主键

在基础语法中，我们学会了如何定义类 (Class)。但在 RBQ 中，并不是所有的类都会直接变成数据库里的一张表。

## 1. 实体表 (Entity Table) vs 自由对象 (Free Object)

这是 RBQ 建模中最重要的概念：

*   **实体表**：标记了 `@table` 注解的类。它们会被映射为数据库中的物理表，拥有独立的生命周期。
*   **自由对象**：没有标记 `@table` 的类。它们通常作为其他表的一个字段（通常以 JSON 格式存储），或者被“平铺”到父表中。

### 1.1 定义一个实体

```rbq
@table(name = "users")
class User {
    @key
    id: uuid;
    username: string;
}
```

*   **@table(name = "...")**：告诉编译器这是一个实体，并指定物理表名。
*   **@key**：标记该字段为主键。每个实体表**必须**有一个且仅有一个主键。

### 1.2 定义一个自由对象

```rbq
class Profile {
    avatar_url: string;
    bio: string?;
}

@table(name = "users")
class User {
    @key id: uuid;
    # profile 是一个自由对象，默认在物理表中映射为 JSONB/JSON 列
    profile: Profile;
}
```

## 2. 字段平铺 (@inline)

如果你希望将自由对象的字段直接作为列存放在父表中，而不是存储为 JSON，可以使用 `@inline` 注解。

```rbq
class Location {
    lat: f64;
    lng: f64;
}

@table(name = "shops")
class Shop {
    @key id: uuid;
    name: string;
    
    @inline
    pos: Location; # 物理列将包含: pos_lat, pos_lng
}
```

## 3. 索引与复合约束 (Indexes & Constraints)

除了主键，你还可以定义唯一约束和查询索引，以保证数据一致性和查询性能。

### 3.1 唯一约束 (@unique)
*   **单列唯一**: 直接放在字段上。
*   **复合唯一**: 放在 `class` 上，并指定字段列表。

```rbq
@table(name = "users")
@unique(["org_id", "employee_no"]) # 复合唯一约束
class User {
    @key id: uuid;
    
    @unique
    email: string; # 单列唯一

    org_id: uuid;
    employee_no: string;
}
```

### 3.2 索引 (@index)
通过 `@index` 优化查询速度。

```rbq
@table(name = "posts")
@index(["status", "created_at"], name = "idx_status_time")
class Post {
    @key id: uuid;
    status: string;
    created_at: datetime;
}
```

## 4. 虚拟字段与计算列 (Virtual & Computed)

有些字段并不需要直接存储在数据库中，或者它们是由其他字段计算得出的。

### 4.1 虚拟字段 (@virtual)
虚拟字段仅存在于内存中，不会映射为数据库列。它们通常用于业务逻辑或动态展示。

```rbq
class User {
    first_name: string;
    last_name: string;
    
    @virtual
    full_name: string; # 仅在逻辑层可用
}
```

### 4.2 物理计算列 (@computed)
计算列在数据库层实现，由数据库自动维护。

```rbq
class OrderItem {
    price: d128;
    quantity: i32;
    
    @computed(formula = "price * quantity")
    total: d128; # 映射为数据库的 GENERATED ALWAYS 列
}
```

## 5. 默认值

你可以为字段指定默认值，这些值在插入数据时如果缺省则会自动填充。

```rbq
class Config {
    @key id: uuid;
    # 使用常量或表达式作为默认值
    theme: string = "dark";
    retry_count: i32 = 3;
}
```

## 6. RPC 相关注解

除了数据库相关的注解，RBQ 还提供了一系列用于 RPC 服务的注解：

### 6.1 服务方法注解

| 注解 | 适用范围 | 说明 |
|------|----------|------|
| `@timeout(duration)` | 服务方法 | 设置 RPC 超时，如 `@timeout(5s)` |
| `@retry(count)` | 服务方法 | 设置重试次数，如 `@retry(3)` |
| `@middleware(name)` | 服务方法 | 应用中间件，如 `@middleware("auth")` |

### 6.2 消息字段注解

| 注解 | 适用范围 | 说明 |
|------|----------|------|
| `@required` | 消息字段 | 必填，生成验证代码 |
| `@validation(min=1, max=100)` | 字段 | 数值范围验证 |
| `@pattern(regex)` | 字段 | 正则验证 |

### 6.3 使用示例

```rbq
// 消息字段注解
message CreateUserRequest {
    @required
    username: string;
    
    @validation(min=6, max=50)
    password: string;
    
    @pattern("^[a-zA-Z0-9+_.-]+@[a-zA-Z0-9.-]+$")
    email: string;
}

// 服务方法注解
service UserService {
    @timeout(5s)
    @middleware("auth")
    create_user(request: CreateUserRequest) -> User;
    
    @retry(3)
    get_user(request: GetUserRequest) -> User;
}
```

---

**小结**：
*   想生成表？用 `@table`。
*   主键？用 `@key`。
*   嵌套数据不想用 JSON？用 `@inline`。
*   RPC 方法超时？用 `@timeout`。
*   RPC 方法重试？用 `@retry`。
*   RPC 方法中间件？用 `@middleware`。
*   消息字段必填？用 `@required`。

**下一节**：我们将学习 RBQ 强大的代数数据类型 (ADT)，看看它是如何优雅地处理枚举和多态数据的。
