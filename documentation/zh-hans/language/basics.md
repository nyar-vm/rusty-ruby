# 快速上手：基础语法

欢迎来到 RBQ 的第一个代码实践环节。RBQ 的语法设计追求极简、直观，且对 Rust 开发者天然友好。

## 1. 注释 (Comments)

RBQ 支持两种注释方式：
- 单行注释：以 `//` 开头
- 块注释：以 `/*` 开头，以 `*/` 结尾

```rbq
// 这是一个合法的单行注释
/* 这是一个合法的
   块注释 */
model User {
    id: i64;
}
```

## 2. 标量类型 (Scalar Types)

RBQ 提供了一套严谨的强类型系统：

*   **整数**：`i8`, `i16`, `i32`, `i64` (有符号)；`u8`, `u16`, `u32`, `u64`, `u128` (无符号)。
*   **浮点数**：`f32`, `f64`。
*   **文本**：`string` (UTF-8 编码的变长字符串)。
*   **逻辑**：`bool` (true 或 false)。
*   **二进制**：`bytes` (二进制数据)。
*   **时间与日期**：`datetime` (带时区的时间戳)。
*   **集合类型**：`[]T` (数组)，`map<K,V>` (映射)。

## 3. 定义数据模型 (Model)

使用 `model` 关键字定义数据模型。每个字段由 `[pub] <字段名>: <类型> [= <默认值>] [;]` 组成。

```rbq
model User {
    // 字段定义： field: type = default
    id: i64 = 0;                 // 默认值 0
    username: string;             // 无默认值
    email: string;
    created_at: datetime = now(); // 支持内置函数 now(), uuid() 等

    // 注解：@annotation
    @primary_key
    @auto_increment
    id: i64;                  // 可选，默认生成字段为 pub

    @unique
    username: string;

    @index
    created_at: datetime;

    // 关系
    posts: Post[];                 // 一对多，通过外键关联
}
```

### 3.1 可选字段 (Option Types)

如果某个字段可能为空，在类型后添加 `?` 或使用 `Option<T>`。

```rbq
model User {
    id: i64;
    nickname: string?;      # 昵称是可选的
    age: Option<i32>;     # 与 string? 语义一致
}
```

## 4. 定义类 (Class)

使用 `class` 关键字定义更通用的类型。类可以分为两种：
- **实体表**：带有 `@table` 注解，映射到数据库物理表
- **自由对象**：无 `@table` 注解，作为其他表的字段（通常以 JSON 格式存储）或被“平铺”到父表中

```rbq
// 定义一个自由对象
class Profile {
    avatar_url: string;
    bio: string?;
}

// 定义一个实体表
@table(name = "users")
class User {
    @key
    id: uuid;
    username: string;
    profile: Profile; // 自由对象作为字段
}
```

## 5. 定义消息 (Message)

使用 `message` 关键字定义 RPC 请求/响应消息。语法同模型，但无数据库注解。

```rbq
message GetUserRequest {
    id: i64;
    @required
    id: i64;                       // 注解可用于消息字段
}

message UserList {
    users: User[];
    total: i64;
}
```

## 6. 定义服务 (Service)

使用 `service` 关键字定义 RPC 服务，包含一组 RPC 方法。方法语法：`method_name(args) -> return_type [;]`

```rbq
service UserService {
    // 一元 RPC
    get_user(request: GetUserRequest) -> User;

    // 服务端流：返回 stream T
    list_users(request: ListUsersRequest) -> stream User;

    // 客户端流：参数为 stream T
    create_users(requests: stream CreateUserRequest) -> BatchResponse;

    // 双向流
    chat(messages: stream ChatMessage) -> stream ChatMessage;

    // 可附加注解
    @timeout(5s)
    @middleware("auth")
    delete_user(request: DeleteUserRequest) -> DeleteResponse;
}
```

## 7. 定义枚举 (Enum)

使用 `enum` 关键字定义枚举类型。枚举值必须指定整数常量。

```rbq
enum Status {
    ACTIVE = 0;
    INACTIVE = 1;
    BANNED = 2;
}

model Account {
    status: Status;
}
```

## 8. 导入机制 (Using)

使用 `using` 语句导入其他文件中定义的模型、消息、枚举和服务。

```rbq
// user.rbq
using common.User;           // 导入 common.rbq 中的 User 模型
using common.*;              // 导入 common 中所有公开定义

model ExtendedUser {
    base: User;              // 使用导入的模型
    extra: string;
}
```

## 9. 可见性

默认情况下，模型、消息、服务是私有的（仅文件内可见）。使用 `pub` 关键字导出，使其可被其他 rbq 文件 `using`。

```rbq
pub model PublicModel { ... }
```

---

**下一节**：我们将学习如何将这些结构体映射到真正的数据库表中，并理解“实体”的概念。
