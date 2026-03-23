//! 虚拟机测试

use ruby::Ruby;
use ruby_types::{RubyError, RubyValue};

#[test]
fn test_basic_arithmetic() {
    let mut ruby = Ruby::new().unwrap();

    // 直接设置全局变量
    ruby.set_global("$a", RubyValue::Integer(10)).unwrap();
    ruby.set_global("$b", RubyValue::Integer(5)).unwrap();

    // 定义 add 方法
    ruby.define_global_method(
        "add",
        Box::new(|_ctx, args| {
            if args.len() >= 2 {
                let sum = args[0].to_f64() + args[1].to_f64();
                Ok(RubyValue::Float(sum))
            }
            else {
                Err(RubyError::ArgumentError("Not enough arguments for add method".to_string()))
            }
        }),
    )
    .unwrap();

    // 定义 subtract 方法
    ruby.define_global_method(
        "subtract",
        Box::new(|_ctx, args| {
            if args.len() >= 2 {
                let difference = args[0].to_f64() - args[1].to_f64();
                Ok(RubyValue::Float(difference))
            }
            else {
                Err(RubyError::ArgumentError("Not enough arguments for subtract method".to_string()))
            }
        }),
    )
    .unwrap();

    // 直接使用VM API测试方法调用
    let a = ruby.get_global("$a").unwrap();
    let b = ruby.get_global("$b").unwrap();

    // 模拟方法调用
    let context = ruby.vm().context();
    let mut context = context.lock().unwrap();

    let sum = context.call_method("add", vec![a.clone(), b.clone()]).unwrap();
    let difference = context.call_method("subtract", vec![a, b]).unwrap();

    // 检查结果
    assert_eq!(sum, RubyValue::Float(15.0));
    assert_eq!(difference, RubyValue::Float(5.0));
}

#[test]
fn test_variable_assignment() {
    let mut ruby = Ruby::new().unwrap();

    // 直接设置全局变量
    ruby.set_global("$global_var", RubyValue::Integer(42)).unwrap();

    // 检查全局变量
    let global_var = ruby.get_global("$global_var").unwrap();
    assert_eq!(global_var, RubyValue::Integer(42));
}

#[test]
fn test_method_call() {
    let mut ruby = Ruby::new().unwrap();

    // 定义 say_hello 方法
    ruby.define_global_method(
        "say_hello",
        Box::new(|_ctx, args| {
            let name = if args.len() > 0 { args[0].to_string() } else { "World".to_string() };
            Ok(RubyValue::String(format!("Hello, {}", name)))
        }),
    )
    .unwrap();

    // 直接使用VM API测试方法调用
    let context = ruby.vm().context();
    let mut context = context.lock().unwrap();

    let greeting = context.call_method("say_hello", vec![RubyValue::String("Ruby".to_string())]).unwrap();

    // 检查结果
    assert_eq!(greeting, RubyValue::String("Hello, Ruby".to_string()));
}
