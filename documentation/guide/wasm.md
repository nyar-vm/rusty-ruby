# Rusty Ruby WebAssembly 支持指南

## 1. 概述

Rusty Ruby 提供了 WebAssembly (WASM) 支持，使得 Ruby 代码可以在 Web 浏览器和其他 WebAssembly 运行时环境中执行。本指南将详细介绍 WebAssembly 支持的 API 接口和使用方法。

## 2. 核心 API

### 2.1 RubyWASI 类

`RubyWASI` 是 Rusty Ruby WebAssembly 支持的核心类，它提供了在 WebAssembly 环境中执行 Ruby 代码的能力。

```rust
#[wasm_bindgen]
pub struct RubyWASI {
    runtime: Ruby,
    modules: HashMap<String, WasmModule>,
    memory: HashMap<String, JsValue>,
    functions: HashMap<String, js_sys::Function>,
    event_handlers: HashMap<String, Vec<EventHandler>>,
}
```

## 3. 基本操作

### 3.1 创建 RubyWASI 实例

```javascript
// 在 JavaScript 中创建 RubyWASI 实例
const wasi = new RubyWASI();
```

### 3.2 执行 Ruby 代码

```javascript
// 执行 Ruby 代码
const result = wasi.evaluate('puts "Hello, WebAssembly!"');
console.log(result); // 输出: "Execution successful"
```

### 3.3 重置 Ruby 运行时

```javascript
// 重置 Ruby 运行时
wasi.reset();
```

## 4. 模块管理

### 4.1 加载 WebAssembly 模块

```javascript
// 加载 WebAssembly 模块
const moduleData = new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]);
try {
    const module = wasi.loadModule('test_module', moduleData);
    console.log('Module loaded:', module.moduleName());
} catch (error) {
    console.error('Failed to load module:', error);
}
```

### 4.2 卸载 WebAssembly 模块

```javascript
// 卸载 WebAssembly 模块
const success = wasi.unloadModule('test_module');
console.log('Module unloaded:', success);
```

## 5. 内存管理

### 5.1 设置内存数据

```javascript
// 设置内存数据
wasi.setMemory('test_key', 'test_value');
```

### 5.2 获取内存数据

```javascript
// 获取内存数据
const value = wasi.getMemory('test_key');
console.log('Memory value:', value);

// 获取不存在的内存数据
const nonExistent = wasi.getMemory('non_existent_key');
console.log('Non-existent value:', nonExistent); // 输出: null
```

## 6. 函数调用

### 6.1 注册函数

```javascript
// 注册函数
const testFunc = function() {
    return 'test_result';
};
wasi.registerFunction('test_function', testFunc);
```

### 6.2 调用函数

```javascript
// 调用函数
const args = [];
try {
    const result = wasi.callFunction('test_function', args);
    console.log('Function result:', result); // 输出: "test_result"
} catch (error) {
    console.error('Failed to call function:', error);
}

// 调用不存在的函数
try {
    const result = wasi.callFunction('non_existent_function', args);
} catch (error) {
    console.error('Expected error:', error);
}
```

## 7. 事件处理

### 7.1 注册事件处理器

```javascript
// 注册事件处理器
const eventHandler = function(eventName, data) {
    console.log('Event:', eventName, data);
};
wasi.onEvent('test_event', eventHandler);
```

### 7.2 触发事件

```javascript
// 触发事件
const eventData = 'test_data';
wasi.triggerEvent('test_event', eventData);
```

### 7.3 移除事件处理器

```javascript
// 移除事件处理器
wasi.offEvent('test_event', eventHandler);

// 再次触发事件（不会调用已移除的处理器）
wasi.triggerEvent('test_event', 'should not be logged');
```

## 8. 完整示例

### 8.1 在浏览器中使用 RubyWASI

```html
<!DOCTYPE html>
<html>
<head>
    <title>Rusty Ruby WebAssembly Example</title>
</head>
<body>
    <h1>Rusty Ruby WebAssembly Example</h1>
    <div id="output"></div>

    <script type="module">
        // 导入 Rusty Ruby WebAssembly 模块
        import init from './pkg/rusty_ruby_wasm.js';

        async function run() {
            // 初始化 WebAssembly 模块
            await init();

            // 创建 RubyWASI 实例
            const wasi = new window.RubyWASI();
            const output = document.getElementById('output');

            // 执行 Ruby 代码
            const result = wasi.evaluate('puts "Hello from Ruby!"');
            output.innerHTML += `<p>Execute result: ${result}</p>`;

            // 注册和调用函数
            const addFunc = function(a, b) {
                return a + b;
            };
            wasi.registerFunction('add', addFunc);

            const args = [5, 3];
            const addResult = wasi.callFunction('add', args);
            output.innerHTML += `<p>5 + 3 = ${addResult}</p>`;

            // 事件处理
            const eventHandler = function(eventName, data) {
                output.innerHTML += `<p>Event ${eventName} triggered with data: ${data}</p>`;
            };
            wasi.onEvent('test_event', eventHandler);

            // 触发事件
            wasi.triggerEvent('test_event', 'Hello from event!');

            // 内存管理
            wasi.setMemory('user', { name: 'John', age: 30 });
            const user = wasi.getMemory('user');
            output.innerHTML += `<p>User: ${JSON.stringify(user)}</p>`;
        }

        run().catch(console.error);
    </script>
</body>
</html>
```

### 8.2 在 Node.js 中使用 RubyWASI

```javascript
// 导入 Rusty Ruby WebAssembly 模块
const { RubyWASI } = require('./pkg/rusty_ruby_wasm.js');

// 创建 RubyWASI 实例
const wasi = new RubyWASI();

// 执行 Ruby 代码
const result = wasi.evaluate('puts "Hello from Ruby!"');
console.log('Execute result:', result);

// 注册和调用函数
const multiplyFunc = function(a, b) {
    return a * b;
};
wasi.registerFunction('multiply', multiplyFunc);

const args = [4, 5];
const multiplyResult = wasi.callFunction('multiply', args);
console.log('4 * 5 =', multiplyResult);

// 事件处理
const eventHandler = function(eventName, data) {
    console.log('Event', eventName, 'triggered with data:', data);
};
wasi.onEvent('test_event', eventHandler);

// 触发事件
wasi.triggerEvent('test_event', 'Hello from event!');

// 内存管理
wasi.setMemory('config', { debug: true, level: 'info' });
const config = wasi.getMemory('config');
console.log('Config:', config);

// 重置运行时
wasi.reset();
console.log('Runtime reset');
```

## 9. 最佳实践

1. **模块管理**：合理管理 WebAssembly 模块的加载和卸载，避免内存泄漏
2. **内存使用**：注意内存使用，避免存储过大的数据在内存中
3. **事件处理**：及时移除不再需要的事件处理器，避免内存泄漏
4. **错误处理**：总是处理可能的错误，特别是在函数调用和模块加载时
5. **性能优化**：对于频繁调用的场景，考虑使用缓存和批处理

## 10. 性能优化

### 10.1 减少跨边界调用

- 减少 JavaScript 和 WebAssembly 之间的频繁调用
- 批量处理数据，减少边界 crossings

### 10.2 优化内存使用

- 合理使用内存存储，避免频繁的内存分配和释放
- 使用适当的数据结构，减少内存拷贝

### 10.3 事件处理优化

- 避免在事件处理器中执行复杂操作
- 考虑使用事件节流和防抖技术

## 11. 常见问题

### 11.1 模块加载失败

**问题**：`loadModule` 方法抛出错误

**解决方案**：确保提供的模块数据是有效的 WebAssembly 模块二进制数据

### 11.2 函数调用失败

**问题**：`callFunction` 方法抛出错误

**解决方案**：确保函数已注册，并且参数类型正确

### 11.3 内存使用过高

**问题**：内存使用持续增长

**解决方案**：及时清理不再需要的内存数据和事件处理器

## 12. 总结

Rusty Ruby 的 WebAssembly 支持为 Ruby 代码在 Web 环境中的执行提供了强大的能力。通过本指南的学习，您应该能够熟练使用 WebAssembly 支持的各种 API 接口，为 Ruby 应用程序在 Web 平台上的部署和运行提供有力支持。