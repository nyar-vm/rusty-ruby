# xRPC 概念

## 什么是 xRPC

xRPC 是一种高性能 RPC 框架概念，它是 RBQ 的核心设计理念之一。xRPC 旨在为构建高性能的微服务提供完整的工具链，强调简洁、高效和灵活。

## xRPC 的设计理念

1. **高性能**：采用无锁设计、零拷贝序列化，充分利用 Rust 的异步生态。
2. **简洁协议**：自定义简单帧协议，避免 HTTP/2 的复杂性。
3. **灵活扩展**：模块化设计，支持多种传输层和序列化方式。
4. **与 ORM 集成**：作为 RBQ DSL 的一部分，实现模型与 RPC 服务的统一定义。

## xRPC 的分层架构

| 层级 | 职责 | 关键组件 |
|------|------|----------|
| **传输层** | 管理底层连接和数据帧传输 | `FrameTransport` trait（支持 TCP、Unix Domain Socket、共享内存） |
| **消息层** | 序列化/反序列化、压缩、元信息透传 | 基于 Prost（Protobuf）但可插拔，支持 LZ4/Zstd 压缩 |
| **RPC 层** | 方法派发、中间件链、流控 | `Service` trait（AFIT/RPITIT 实现零开销抽象），支持一元、客户端流、服务端流、双向流 |
| **治理层** | 负载均衡、服务发现、灰度路由 | 内置轮询、一致性哈希，与 xRPC 命名服务集成 |

## xRPC 的协议设计

xRPC 采用自定义的简单帧协议：

- **帧格式**：`[长度:4字节][消息体]`，消息体为 Protobuf 编码的 `xrpc.Request` / `xrpc.Response` 封装。
- **多路复用**：每个连接可并发多个 stream，通过 stream ID 区分。
- **流控**：基于窗口的动态流控，防止接收端过载。

## xRPC 的性能优势

- **无锁传输**：核心路径避免使用 `std::sync::Mutex`，采用原子操作和 channel 设计。
- **零拷贝**：尽可能使用 `Bytes` 共享数据，减少内存复制。
- **异步原生**：基于 `tokio`，充分利用 Rust 异步生态。
- **零开销抽象**：所有生成的代码均使用静态分发，无虚函数调用。
- **内存效率**：使用 `Bytes` 避免数据复制，序列化/反序列化直接操作字节缓冲区。

## xRPC 与 RBQ 的关系

xRPC 是 RBQ 的核心设计理念之一，RBQ 通过以下方式实现 xRPC 概念：

1. **DSL 集成**：在 `.rbq` 文件中统一定义数据模型和 RPC 服务。
2. **代码生成**：编译器 `rbqc` 生成高性能的 RPC 客户端和服务器代码。
3. **类型安全**：所有 RPC 方法和消息类型都是类型安全的。
4. **多种通信模式**：支持一元、客户端流、服务端流、双向流等通信模式。
5. **中间件支持**：通过注解系统，支持认证、日志、监控等中间件。

## 示例

```rbq
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