#[cfg(test)]
mod tests {
    use crate::RubyWASI;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::wasm_bindgen_test;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_load_unload_module() {
        let mut wasi = RubyWASI::new();
        let module_data = &[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]; // 简单的 WASM 模块头

        // 测试加载模块
        let result = wasi.load_module("test_module", module_data);
        assert!(result.is_ok());
        let module = result.unwrap();
        assert_eq!(module.module_name(), "test_module");

        // 测试卸载模块
        assert!(wasi.unload_module("test_module"));
        assert!(!wasi.unload_module("non_existent_module"));
    }

    #[wasm_bindgen_test]
    fn test_memory_operations() {
        let mut wasi = RubyWASI::new();
        let test_value = JsValue::from_str("test_value");

        // 测试设置内存
        wasi.set_memory("test_key", test_value.clone());

        // 测试获取内存
        let result = wasi.get_memory("test_key");
        assert_eq!(result, test_value);

        // 测试获取不存在的内存
        let result = wasi.get_memory("non_existent_key");
        assert!(result.is_null());
    }

    #[wasm_bindgen_test]
    fn test_function_operations() {
        let mut wasi = RubyWASI::new();

        // 创建一个测试函数
        let test_func = js_sys::Function::new_no_args("return 'test_result'");

        // 测试注册函数
        wasi.register_function("test_function", test_func);

        // 测试调用函数
        let args = js_sys::Array::new();
        let result = wasi.call_function("test_function", &args.into());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), JsValue::from_str("test_result"));

        // 测试调用不存在的函数
        let args2 = js_sys::Array::new();
        let result = wasi.call_function("non_existent_function", &args2.into());
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_event_operations() {
        let mut wasi = RubyWASI::new();
        let mut event_called = false;

        // 创建一个测试事件处理器
        let handler = js_sys::Function::new_with_args("event_name, data", "console.log('Event:', event_name, data);");

        // 测试注册事件处理器
        wasi.on_event("test_event", handler.clone());

        // 测试触发事件
        let event_data = JsValue::from_str("test_data");
        wasi.trigger_event("test_event", &event_data);

        // 测试移除事件处理器
        wasi.off_event("test_event", handler);

        // 测试触发已移除的事件
        wasi.trigger_event("test_event", &event_data);
    }

    #[wasm_bindgen_test]
    fn test_reset() {
        let mut wasi = RubyWASI::new();
        let module_data = &[0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];

        // 加载模块
        wasi.load_module("test_module", module_data).unwrap();

        // 设置内存
        wasi.set_memory("test_key", JsValue::from_str("test_value"));

        // 注册函数
        let test_func = js_sys::Function::new_no_args("return 'test_result'");
        wasi.register_function("test_function", test_func);

        // 注册事件处理器
        let handler = js_sys::Function::new_with_args("event_name, data", "console.log('Event:', event_name, data);");
        wasi.on_event("test_event", handler);

        // 重置
        wasi.reset();

        // 验证所有数据都被清除
        assert!(!wasi.unload_module("test_module")); // 模块已被清除
        assert!(wasi.get_memory("test_key").is_null()); // 内存已被清除

        // 验证函数已被清除
        let args = js_sys::Array::new();
        let result = wasi.call_function("test_function", &args.into());
        assert!(result.is_err());
    }
}
