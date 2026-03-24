# Rusty Ruby 类型系统指南

## 1. 概述

Rusty Ruby 类型系统提供了一套完整的 Ruby 值类型和操作方法，用于在 Rust 中表示和操作 Ruby 数据。本指南将详细介绍类型系统的使用方法和最佳实践。

## 2. 核心类型

Rusty Ruby 类型系统的核心是 `RubyValue` 枚举，它表示所有可能的 Ruby 值类型：

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RubyValue {
    /// Nil 值
    Nil,
    /// 布尔值
    Boolean(bool),
    /// 整数值
    Integer(i32),
    /// 浮点值
    Float(f64),
    /// 字符串值
    String(String),
    /// 符号值
    Symbol(String),
    /// 数组值
    Array(Vec<RubyValue>),
    /// 哈希值
    Hash(std::collections::HashMap<String, RubyValue>),
    /// 对象值
    Object(String, std::collections::HashMap<String, RubyValue>),
    /// 闭包值
    Closure(usize),
}
```

## 3. 基本操作

### 3.1 类型检查

```rust
let value = RubyValue::Integer(42);
assert!(!value.is_nil());
```

### 3.2 类型转换

```rust
let int_value = RubyValue::Integer(42);
let float_value = RubyValue::Float(3.14);
let bool_value = RubyValue::Boolean(true);

assert_eq!(int_value.to_i32(), 42);
assert_eq!(float_value.to_f64(), 3.14);
assert_eq!(bool_value.to_bool(), true);
assert_eq!(int_value.to_string(), "42");
```

## 4. 数组操作

### 4.1 创建数组

```rust
let mut arr = RubyValue::Array(vec![RubyValue::Integer(1), RubyValue::Integer(2)]);
```

### 4.2 基本数组操作

```rust
// 添加元素到数组末尾
arr.push(RubyValue::Integer(3));
assert_eq!(arr.array_length(), Some(3));

// 从数组末尾移除元素
let popped = arr.pop();
assert_eq!(popped, Some(RubyValue::Integer(3)));
assert_eq!(arr.array_length(), Some(2));

// 从数组开头移除元素
let shifted = arr.shift();
assert_eq!(shifted, Some(RubyValue::Integer(1)));
assert_eq!(arr.array_length(), Some(1));

// 添加元素到数组开头
arr.unshift(RubyValue::Integer(0));
assert_eq!(arr.array_length(), Some(2));

// 删除指定索引的元素
let deleted = arr.delete_at(1);
assert_eq!(deleted, Some(RubyValue::Integer(2)));
assert_eq!(arr.array_length(), Some(1));

// 在指定索引插入元素
arr.insert(1, RubyValue::Integer(1));
assert_eq!(arr.array_length(), Some(2));
```

### 4.3 访问数组元素

```rust
let arr = RubyValue::Array(vec![RubyValue::Integer(0), RubyValue::Integer(1)]);

// 获取数组长度
assert_eq!(arr.array_length(), Some(2));

// 获取指定索引的元素
assert_eq!(arr.array_get(0), Some(RubyValue::Integer(0)));

// 修改指定索引的元素
let mut arr_mut = arr.clone();
arr_mut.array_set(1, RubyValue::Integer(2));
assert_eq!(arr_mut.array_get(1), Some(RubyValue::Integer(2)));
```

## 5. 哈希操作

### 5.1 创建哈希

```rust
let mut hash = RubyValue::Hash(std::collections::HashMap::from([
    ("a".to_string(), RubyValue::Integer(1)),
    ("b".to_string(), RubyValue::Integer(2)),
]));
```

### 5.2 基本哈希操作

```rust
// 获取哈希值
assert_eq!(hash.hash_get("a"), Some(RubyValue::Integer(1)));

// 设置哈希值
hash.hash_set("c", RubyValue::Integer(3));
assert_eq!(hash.hash_get("c"), Some(RubyValue::Integer(3)));

// 删除哈希键
let deleted = hash.delete("a");
assert_eq!(deleted, Some(RubyValue::Integer(1)));
assert_eq!(hash.hash_get("a"), None);

// 清空哈希
hash.clear();
assert_eq!(hash.hash_keys(), Some(vec![]));

// 合并哈希
let mut hash1 = RubyValue::Hash(std::collections::HashMap::from([
    ("a".to_string(), RubyValue::Integer(1)),
]));
let hash2 = RubyValue::Hash(std::collections::HashMap::from([
    ("b".to_string(), RubyValue::Integer(2)),
]));
hash1.merge(&hash2);
assert_eq!(hash1.hash_get("b"), Some(RubyValue::Integer(2)));
```

### 5.3 哈希查询

```rust
let hash = RubyValue::Hash(std::collections::HashMap::from([
    ("a".to_string(), RubyValue::Integer(1)),
    ("b".to_string(), RubyValue::Integer(2)),
    ("c".to_string(), RubyValue::Integer(3)),
]));

// 选择符合条件的键值对
let selected = hash.select(|_, v| v.to_i32() > 1);
if let Some(RubyValue::Hash(map)) = selected {
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("b"), Some(&RubyValue::Integer(2)));
    assert_eq!(map.get("c"), Some(&RubyValue::Integer(3)));
}

// 拒绝符合条件的键值对
let rejected = hash.reject(|_, v| v.to_i32() > 1);
if let Some(RubyValue::Hash(map)) = rejected {
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("a"), Some(&RubyValue::Integer(1)));
}
```

## 6. 字符串操作

### 6.1 创建字符串

```rust
let s = RubyValue::String("Hello World".to_string());
```

### 6.2 基本字符串操作

```rust
// 获取字符串长度
assert_eq!(s.length(), Some(11));

// 截取字符串
assert_eq!(s.slice(0, Some(5)), Some(RubyValue::String("Hello".to_string())));
assert_eq!(s.slice(6, None), Some(RubyValue::String("World".to_string())));

// 分割字符串
let parts = s.split(" ");
if let Some(RubyValue::Array(arr)) = parts {
    assert_eq!(arr.len(), 2);
    assert_eq!(arr[0], RubyValue::String("Hello".to_string()));
    assert_eq!(arr[1], RubyValue::String("World".to_string()));
}

// 转换为大写
assert_eq!(s.upcase(), Some(RubyValue::String("HELLO WORLD".to_string())));

// 转换为小写
assert_eq!(s.downcase(), Some(RubyValue::String("hello world".to_string())));
```

## 7. 数值操作

### 7.1 创建数值

```rust
let int_value = RubyValue::Integer(-5);
let float_value = RubyValue::Float(-3.14);
```

### 7.2 基本数值操作

```rust
// 获取绝对值
assert_eq!(int_value.abs(), Some(RubyValue::Integer(5)));
assert_eq!(float_value.abs(), Some(RubyValue::Float(3.14)));

// 四舍五入
let float = RubyValue::Float(3.6);
assert_eq!(float.round(), Some(RubyValue::Float(4.0)));

// 向下取整
assert_eq!(float.floor(), Some(RubyValue::Float(3.0)));

// 向上取整
assert_eq!(float.ceil(), Some(RubyValue::Float(4.0)));

// 计算平方根
let positive_int = RubyValue::Integer(4);
let positive_float = RubyValue::Float(9.0);
assert_eq!(positive_int.sqrt(), Some(RubyValue::Float(2.0)));
assert_eq!(positive_float.sqrt(), Some(RubyValue::Float(3.0)));
```

## 8. 对象操作

### 8.1 创建对象

```rust
let mut obj = RubyValue::Object(
    "Person".to_string(),
    std::collections::HashMap::from([
        ("name".to_string(), RubyValue::String("John".to_string())),
        ("age".to_string(), RubyValue::Integer(30)),
    ]),
);
```

### 8.2 访问和修改对象字段

```rust
// 获取对象字段
assert_eq!(obj.object_get("name"), Some(RubyValue::String("John".to_string())));

// 修改对象字段
obj.object_set("age", RubyValue::Integer(31));
assert_eq!(obj.object_get("age"), Some(RubyValue::Integer(31)));
```

## 9. 最佳实践

1. **类型检查**：在操作值之前，始终检查其类型以避免运行时错误
2. **内存管理**：注意大对象的内存使用，特别是数组和哈希
3. **性能优化**：对于频繁操作的场景，考虑使用更高效的数据结构
4. **错误处理**：处理可能的 `None` 返回值，特别是在类型转换和索引访问时

## 10. 示例：完整的类型操作

```rust
use ruby_types::RubyValue;

fn main() {
    // 创建和操作数组
    let mut arr = RubyValue::Array(vec![RubyValue::Integer(1), RubyValue::Integer(2)]);
    arr.push(RubyValue::Integer(3));
    println!("Array length: {:?}", arr.array_length());
    
    // 创建和操作哈希
    let mut hash = RubyValue::Hash(std::collections::HashMap::new());
    hash.hash_set("key1", RubyValue::String("value1".to_string()));
    hash.hash_set("key2", RubyValue::Integer(42));
    println!("Hash value: {:?}", hash.hash_get("key1"));
    
    // 创建和操作字符串
    let s = RubyValue::String("Hello Rusty Ruby".to_string());
    println!("String length: {:?}", s.length());
    println!("Uppercase: {:?}", s.upcase());
    
    // 创建和操作数值
    let num = RubyValue::Integer(-10);
    println!("Absolute value: {:?}", num.abs());
}
```

## 11. 总结

Rusty Ruby 类型系统提供了一套完整的 Ruby 值类型和操作方法，使得在 Rust 中处理 Ruby 数据变得简单和高效。通过本指南的学习，您应该能够熟练使用类型系统的各种功能，为 Rusty Ruby 项目开发提供有力支持。