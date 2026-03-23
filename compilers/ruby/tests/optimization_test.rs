//! 优化测试

use ruby::Ruby;
use ruby_types::RubyValue;

#[test]
fn test_constant_folding() {
    let mut ruby = Ruby::new().unwrap();

    // 测试常量折叠
    let script = r#"
    $result = 1 + 2 * 3
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(7));
}

#[test]
fn test_dead_code_elimination() {
    let mut ruby = Ruby::new().unwrap();

    // 测试死代码消除
    let script = r#"
    $result = 0
    // 这是一个纯表达式，应该被消除
    1 + 2
    $result = 42
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(42));
}

#[test]
fn test_if_constant_folding() {
    let mut ruby = Ruby::new().unwrap();

    // 测试 if 语句的常量折叠
    let script = r#"
    $result = 0
    if true
      $result = 1
    else
      $result = 2
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(1));
}

#[test]
fn test_while_constant_folding() {
    let mut ruby = Ruby::new().unwrap();

    // 测试 while 语句的常量折叠
    let script = r#"
    $result = 0
    while false
      $result = 1
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(0));
}
