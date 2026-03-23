# 安全与隔离：策略与租户

在多租户系统或对安全性要求极高的应用中，仅仅依靠应用层（如 Rust 代码）来过滤数据是不够的。RBQ 将安全策略直接内置到了 Schema 定义中。

## 1. 行级安全策略 (@policy)

你可以为实体定义细粒度的读写权限。

```rbq
@policy(
    read = "$owner_id == context.user_id",
    write = "context.has_role('admin') || $owner_id == context.user_id"
)
class PrivateNote {
    @key id: uuid;
    content: utf8;
    owner_id: uuid;
}
```

**效果**：
当开发者编写 `PrivateNote.list()` 时，RBQ 编译器会自动根据当前上下文（Context）注入过滤条件。这意味着：**即使开发者忘记写 filter，普通用户也绝对查不到别人的笔记。**

## 2. 原生多租户 (@tenant)

多租户隔离是 SaaS 应用的基石。

```rbq
@tenant
class Client {
    @key id: uuid;
    name: utf8;
}
```

**效果**：
1.  **物理层**：自动注入 `tenant_id` 字段并建立索引。
2.  **执行层**：所有的查询和写入操作都会强制绑定当前 Session 的租户 ID。
3.  **安全性**：彻底杜绝了因代码逻辑漏洞导致的跨租户数据泄露。

## 3. 隐私保护与脱敏 (@mask)

在处理敏感信息（如手机号、身份证、邮箱）时，为了防止在日志或 UI 中泄露，可以使用 `@mask`。

```rbq
class User {
    @key id: uuid;
    
    @mask(pattern = "email")
    email: utf8;
    
    @mask(pattern = "phone")
    mobile: utf8;
}
```

**效果**: 除非在查询时显式请求原始值，否则 RBQ 在投影和序列化这些字段时会自动进行脱敏处理（例如 `ali***@example.com`）。

---

## 4. 小结
*   安全是建模的一部分，而不是事后的补丁。
*   `@policy` 让数据自带访问控制逻辑。
*   `@tenant` 让 SaaS 开发变得无比简单。

**下一节**：我们将学习 RBQ 提供的工程化“黑魔法”，包括自动审计、软删除和版本控制。
