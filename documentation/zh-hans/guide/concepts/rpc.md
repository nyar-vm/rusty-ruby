# RPC 概念

## 什么是 RPC

RPC (Remote Procedure Call) 是一种通信协议，允许程序调用另一个地址空间（通常是网络上的另一台计算机）的子程序，而不需要程序员显式编码这个远程调用的细节。

## RPC 的优势

1. **简化分布式编程**：开发者可以像调用本地函数一样调用远程服务，不需要关心网络通信的细节。
2. **提高代码复用**：服务可以被多个客户端共享，提高代码复用率。
3. **解耦服务**：服务之间通过明确的接口进行通信，降低了耦合度。
4. **易于扩展**：可以独立扩展各个服务，提高系统的可扩展性。

## RBQ 中的 RPC 实现

RBQ 内置了基于 xRPC 概念的 RPC 系统，它具有以下特点：

1. **统一定义**：在同一个 `.rbq` 文件中定义数据模型和 RPC 服务，消除了重复定义。
2. **编译期代码生成**：在编译期生成高性能的 RPC 客户端和服务器代码。
3. **类型安全**：所有 RPC 方法和消息类型都是类型安全的，避免运行时错误。
4. **多种通信模式**：支持一元、客户端流、服务端流、双向流等通信模式。
5. **中间件支持**：通过注解系统，支持认证、日志、监控等中间件。
6. **高性能**：采用无锁设计、零拷贝序列化，充分利用 Rust 的异步生态。

## 示例

```rbq
// 消息定义
message GetUserRequest {
    id: i64;
}

message UserList {
    users: User[];
    total: i64;
}

// 服务定义
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

生成的 Rust 代码可以这样使用：

```rust
// 客户端
let client = UserServiceClient::connect("127.0.0.1:8080").await.unwrap();
let user = client.get_user(GetUserRequest { id: 1 }).await.unwrap();

// 服务器实现
struct UserServiceImpl;

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError> {
        // 实现逻辑
    }
    
    async fn list_users(&self, req: ListUsersRequest) -> Result<impl Stream<Item = User>, RpcError> {
        // 实现逻辑
    }
    
    // 其他方法实现...
}

// 启动服务器
let server = serve_user_service(UserServiceImpl);
server.listen("127.0.0.1:8080").await.unwrap();
```