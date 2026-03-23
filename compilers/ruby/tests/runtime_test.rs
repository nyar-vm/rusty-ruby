use ruby::Ruby;
use ruby_types::RubyValue;

#[test]
fn test_execute_script() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");
    
    // 测试执行脚本
    let result = ruby.execute_script("puts 'Hello, Ruby!'");
    assert!(result.is_ok(), "Script execution failed: {:?}", result);
}

#[test]
fn test_global_variables() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");
    
    // 测试设置全局变量
    let value = RubyValue::Integer(42);
    let result = ruby.set_global("$test_var", value);
    assert!(result.is_ok(), "Failed to set global variable: {:?}", result);
    
    // 测试获取全局变量
    let result = ruby.get_global("$test_var");
    assert!(result.is_ok(), "Failed to get global variable: {:?}", result);
    let retrieved_value = result.unwrap();
    assert_eq!(retrieved_value, RubyValue::Integer(42), "Global variable value mismatch");
}

#[test]
fn test_class_definition() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");
    
    // 测试定义类
    let result = ruby.define_class("TestClass");
    assert!(result.is_ok(), "Failed to define class: {:?}", result);
    
    // 测试定义方法
    let result = ruby.define_method("TestClass", "test_method", Box::new(|| {}));
    assert!(result.is_ok(), "Failed to define method: {:?}", result);
}
