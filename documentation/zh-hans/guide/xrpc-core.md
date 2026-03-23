# xRPC 核心设计

xRPC 是一个完全自研、基于 Rust 的高性能 RPC 框架概念，它与 RBQ DSL 深度集成，为构建高性能的微服务提供了完整的工具链。

## 1. 设计哲学

xRPC 的核心设计原则：

- **高性能**：采用无锁设计、零拷贝序列化，充分利用 Rust 的异步生态。
- **简洁协议**：自定义简单帧协议，避免 HTTP/2 的复杂性。
- **灵活扩展**：模块化设计，支持多种传输层和序列化方式。
- **与 RBQ 深度集成**：作为 RBQ DSL 的一部分，实现模型与 RPC 服务的统一定义。

## 2. 为什么选择 xRPC 而非 gRPC

基于纯 Rust，我们完全可以做得比 gRPC 更好——不仅能更好，而且对于追求极致性能和“自研可控”的平台来说，应该做得比 gRPC 更好。

这不是空谈。已经有不少团队在实战中验证了这条路：

- **字节跳动的 Volo 框架**：基于 Rust 最新的 AFIT/RPITIT 特性设计，在 4C 限制下 QPS 达到 35W，优化版本可达 44W，且框架本身开销在火焰图中“基本可以忽略不计”。
- **Momento 的 protosocket**：这家公司原本是 gRPC 重度用户，但发现 `h2` 内部的锁竞争导致任务饥饿。他们自研了极简的 `protosocket`，**垂直扩展能力提升了 2.75 倍**，最终相比 gRPC 实现了 **10 倍以上的性能提升**（按每美元 EC2 成本计算）。
- **xRPC-rs 的分层设计**：展示了从共享内存到 TCP 的无缝切换能力，支持多种序列化协议，且全链路可控。

### 🔍 gRPC 的问题在哪里？

gRPC 是一个非常优秀的通用 RPC 框架，但它有“通用”带来的代价：

1. **HTTP/2 的复杂性**：`h2` 库内部的锁竞争（`std::sync::Mutex`）在高并发下会导致线程被 `futex` 挂起，一次挂起就是几百微秒——这对于追求微秒级响应的系统是巨大开销。
2. **多层抽象的开销**：TLS + HTTP/2 + Protobuf + gRPC 层，每一层都有抽象代价。Rust 虽然零成本抽象，但“零成本”不等于“没有成本”，多层叠加后依然可观。
3. **无法针对场景优化**：你的业务可能是小包高频请求，可能是大包流式传输，但 gRPC 只能提供“通用最优解”，无法给你“场景最优解”。
4. **依赖链冗长**：gRPC-rs 依赖 `tonic`，`tonic` 依赖 `h2` 和 `hyper`，一旦出现性能瓶颈，排查和修改的难度极大。

### 💡 自研 Rust RPC 能带来什么？

#### 1. 极致的性能掌控

**案例：Momento 的 10 倍提升**
Momento 团队发现 gRPC 的瓶颈后，没有在现有框架上打补丁，而是用 Rust 重写了极简的 `protosocket`：
- 移除了 HTTP/2 包装层，直接在 TCP 上定义消息帧
- 用自研的 `k_lock::Mutex` 替换了 `std::sync::Mutex`（更激进的自旋策略）
- 结果：垂直扩展能力提升 2.75 倍，最终达到 **10 倍性能提升**

**这意味着**：同样是 100 台服务器，你家的平台能承载相当于别人 1000 台的服务能力——这就是技术壁垒。

#### 2. 消除不必要的抽象

Rust 的最新特性让我们能写出“真正的零成本抽象”：
- **AFIT（Async Fn in Trait）和 RPITIT（Return Position Impl Trait in Trait）**：Volo 利用这些特性，避免了 `Box` 动态分派，直接在编译期确定所有调用路径。
- **静态分发**：所有的中间件、拦截器都在编译期展开，没有虚函数调用，没有运行时类型识别。

#### 3. 统一 IPC 与 RPC

在平台的演进中（1台→100台），通信模式会经历：
- **阶段1**：单机内进程间通信（共享内存、Unix Domain Socket）
- **阶段2**：同机房多机 TCP 通信
- **阶段3**：跨地域全球通信

gRPC 无法很好地覆盖“单机内”场景——用 TCP 走回环开销太大。而自研框架可以做到：

```rust
// 同一套 API，不同的底层传输
let transport = if cfg!(feature = "shared-memory") {
    SharedMemoryFrameTransport::new("/dev/shm/mos-ipc")?
} else {
    TcpFrameTransport::connect("10.0.0.1:9000")?
};
```

这正是 **xRPC** 的设计思路：从 `ChannelFrameTransport`（同进程测试）到 `SharedMemoryFrameTransport`（生产 IPC）到 `TcpFrameTransport`（网络部署），同一套接口平滑演进。

#### 4. 深度集成生态

自研 RPC 框架可以与你的平台无缝咬合：
- **与监控系统集成**：在框架层自动注入 tracing 上下文，实现全链路追踪
- **与灰度系统集成**：在客户端实现灰度路由策略（如根据用户 ID 哈希选择服务版本）
- **与文件系统集成**：支持大文件流式传输的专用优化路径

这些深度集成在 gRPC 的通用模型下很难优雅实现。

## 3. 分层架构

xRPC 采用清晰的分层架构：

| 层级 | 职责 | 关键组件 |
|------|------|----------|
| **传输层** | 管理底层连接和数据帧传输 | `FrameTransport` trait（支持 TCP、Unix Domain Socket、共享内存） |
| **消息层** | 序列化/反序列化、压缩、元信息透传 | 基于 Prost（Protobuf）但可插拔，支持 LZ4/Zstd 压缩 |
| **RPC 层** | 方法派发、中间件链、流控 | `Service` trait（AFIT/RPITIT 实现零开销抽象），支持一元、客户端流、服务端流、双向流 |
| **治理层** | 负载均衡、服务发现、灰度路由 | 内置轮询、一致性哈希，与 xRPC 命名服务集成 |

## 4. 协议设计

xRPC 采用自定义的简单帧协议：

- **帧格式**：`[长度:4字节][消息体]`，消息体为 Protobuf 编码的 `xrpc.Request` / `xrpc.Response` 封装。
- **多路复用**：每个连接可并发多个 stream，通过 stream ID 区分。
- **流控**：基于窗口的动态流控，防止接收端过载。

## 5. 性能优势

- **无锁传输**：核心路径避免使用 `std::sync::Mutex`，采用原子操作和 channel 设计。
- **零拷贝**：尽可能使用 `Bytes` 共享数据，减少内存复制。
- **异步原生**：基于 `tokio`，充分利用 Rust 异步生态。
- **零开销抽象**：所有生成的代码均使用静态分发，无虚函数调用。
- **内存效率**：使用 `Bytes` 避免数据复制，序列化/反序列化直接操作字节缓冲区。

## 6. 与 RBQ 的集成

xRPC 与 RBQ DSL 深度集成：

- **统一定义**：在同一个 `.rbq` 文件中定义数据模型和 RPC 服务。
- **代码生成**：编译器 `rbqc` 生成完整的 RPC 客户端和服务器代码。
- **类型安全**：所有 RPC 方法和消息类型都是类型安全的。
- **流式支持**：原生支持一元、客户端流、服务端流、双向流等通信模式。

## 7. 服务定义示例

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

## 8. 生成的 RPC 代码

编译器会为每个服务生成对应的客户端和服务器代码：

```rust
// 服务 trait
#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError>;
    async fn list_users(&self, req: ListUsersRequest) -> Result<impl Stream<Item = User>, RpcError>;
    // ...
}

// 客户端实现
pub struct UserServiceClient {
    inner: xrpc::client::Client,
}

impl UserServiceClient {
    pub async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError> {
        self.inner.unary("/user.UserService/get_user", req).await
    }
    // ...
}

// 服务器骨架
pub fn serve_user_service<S: UserService>(service: S) -> xrpc::Server {
    // 注册方法处理器
}
```

## 9. 流类型

对于流式 RPC，生成对应的流包装器：

```rust
type UserStream = xrpc::stream::ReceiverStream<User>;
type ChatMessageStream = xrpc::stream::ReceiverStream<ChatMessage>;
```

## 10. 中间件支持

通过注解系统，xRPC 支持中间件：

- **认证**：验证请求的身份信息。
- **日志**：记录请求和响应。
- **监控**：收集性能指标。
- **限流**：控制请求速率。

## 11. 总结

xRPC 作为一种哲学概念和技术思想，为 RBQ 提供了高性能、灵活的 RPC 能力。它与 RBQ DSL 深度集成，实现了模型与 RPC 服务的统一定义，消除了传统开发中模型重复、更新不一致的痛点。

通过编译期代码生成，xRPC 确保了零开销抽象和类型安全，同时提供了丰富的通信模式支持。这种设计使得开发者可以专注于业务逻辑，同时享受 Rust 带来的极致性能。