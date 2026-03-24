#![warn(missing_docs)]

use ruby::Ruby;
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests;

/// WebAssembly 模块信息
#[wasm_bindgen]
#[derive(Clone)]
pub struct WasmModule {
    module_name: String,
    // 实际实现中可能需要存储模块的其他信息
}

#[wasm_bindgen]
impl WasmModule {
    /// 创建一个新的 WebAssembly 模块
    #[wasm_bindgen(constructor)]
    pub fn new(module_name: &str) -> Self {
        Self { module_name: module_name.to_string() }
    }

    /// 获取模块名称
    #[wasm_bindgen]
    pub fn module_name(&self) -> String {
        self.module_name.to_string()
    }
}

/// 事件处理器类型
type EventHandler = Box<dyn Fn(&str, &JsValue) -> JsValue>;

#[wasm_bindgen]
pub struct RubyWASI {
    runtime: Ruby,
    modules: HashMap<String, WasmModule>,
    memory: HashMap<String, JsValue>,
    functions: HashMap<String, js_sys::Function>,
    event_handlers: HashMap<String, Vec<EventHandler>>,
}

#[wasm_bindgen]
impl RubyWASI {
    /// 创建一个新的 RubyWASI 实例
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let runtime = Ruby::new().unwrap();
        Self { runtime, modules: HashMap::new(), memory: HashMap::new(), functions: HashMap::new(), event_handlers: HashMap::new() }
    }

    /// 评估 Ruby 代码
    #[wasm_bindgen]
    pub fn evaluate(&mut self, code: &str) -> String {
        match self.runtime.execute_script(code) {
            Ok(_) => "Execution successful".to_string(),
            Err(error) => format!("Error: {:?}", error),
        }
    }

    /// 重置 Ruby 运行时
    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.runtime = Ruby::new().unwrap();
        self.modules.clear();
        self.memory.clear();
        self.functions.clear();
        self.event_handlers.clear();
    }

    /// 加载 WebAssembly 模块
    ///
    /// # 参数
    /// * `module_name` - 模块名称
    /// * `module_data` - 模块数据
    ///
    /// # 返回值
    /// 返回加载的模块实例
    #[wasm_bindgen]
    pub fn load_module(&mut self, module_name: &str, module_data: &[u8]) -> Result<WasmModule, JsValue> {
        // 实际实现中，这里应该加载和编译 WebAssembly 模块
        // 目前只是模拟实现
        let module = WasmModule::new(module_name);
        self.modules.insert(module_name.to_string(), module.clone());
        Ok(module)
    }

    /// 卸载 WebAssembly 模块
    ///
    /// # 参数
    /// * `module_name` - 模块名称
    ///
    /// # 返回值
    /// 成功返回 true，失败返回 false
    #[wasm_bindgen]
    pub fn unload_module(&mut self, module_name: &str) -> bool {
        self.modules.remove(module_name).is_some()
    }

    /// 获取内存数据
    ///
    /// # 参数
    /// * `key` - 内存键名
    ///
    /// # 返回值
    /// 返回内存中的值，如果不存在返回 null
    #[wasm_bindgen]
    pub fn get_memory(&self, key: &str) -> JsValue {
        self.memory.get(key).cloned().unwrap_or(JsValue::NULL)
    }

    /// 设置内存数据
    ///
    /// # 参数
    /// * `key` - 内存键名
    /// * `value` - 内存值
    #[wasm_bindgen]
    pub fn set_memory(&mut self, key: &str, value: JsValue) {
        self.memory.insert(key.to_string(), value);
    }

    /// 调用函数
    ///
    /// # 参数
    /// * `function_name` - 函数名称
    /// * `args` - 函数参数
    ///
    /// # 返回值
    /// 返回函数调用结果
    #[wasm_bindgen]
    pub fn call_function(&self, function_name: &str, args: &JsValue) -> Result<JsValue, JsValue> {
        if let Some(func) = self.functions.get(function_name) {
            let args = js_sys::Array::from(args);
            func.apply(&JsValue::NULL, &args)
        }
        else {
            Err(JsValue::from_str(&format!("Function {} not found", function_name)))
        }
    }

    /// 注册函数
    ///
    /// # 参数
    /// * `function_name` - 函数名称
    /// * `func` - 函数对象
    #[wasm_bindgen]
    pub fn register_function(&mut self, function_name: &str, func: js_sys::Function) {
        self.functions.insert(function_name.to_string(), func);
    }

    /// 注册事件处理器
    ///
    /// # 参数
    /// * `event_name` - 事件名称
    /// * `handler` - 事件处理器函数
    #[wasm_bindgen]
    pub fn on_event(&mut self, event_name: &str, handler: js_sys::Function) {
        let handler: EventHandler = Box::new(move |event_name, data| {
            let args = js_sys::Array::new();
            args.push(&JsValue::from_str(event_name));
            args.push(data);
            handler.apply(&JsValue::NULL, &args).unwrap_or(JsValue::NULL)
        });

        self.event_handlers.entry(event_name.to_string()).or_insert_with(Vec::new).push(handler);
    }

    /// 移除事件处理器
    ///
    /// # 参数
    /// * `event_name` - 事件名称
    /// * `handler` - 事件处理器函数
    #[wasm_bindgen]
    pub fn off_event(&mut self, event_name: &str, handler: js_sys::Function) {
        // 注意：实际实现中，这里需要比较函数引用
        // 目前只是模拟实现，移除所有处理器
        self.event_handlers.remove(event_name);
    }

    /// 触发事件
    ///
    /// # 参数
    /// * `event_name` - 事件名称
    /// * `data` - 事件数据
    #[wasm_bindgen]
    pub fn trigger_event(&self, event_name: &str, data: &JsValue) {
        if let Some(handlers) = self.event_handlers.get(event_name) {
            for handler in handlers {
                handler(event_name, data);
            }
        }
    }
}
