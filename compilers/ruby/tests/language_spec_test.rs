//! Ruby 语言规范测试

use ruby::Ruby;
use ruby_types::RubyValue;

#[test]
fn test_basic_arithmetic() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试基本算术运算
    let script = r#"
    $result = 1 + 2
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(3));
}

#[test]
fn test_variable_assignment() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试变量赋值
    let script = r#"
    $x = 10
    $y = 20
    $result = $x + $y
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(30));
}

#[test]
fn test_boolean_operations() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试布尔运算
    let script = r#"
    $a = true
    $b = false
    $result = $a && $b
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Boolean(false));
}

#[test]
fn test_string_operations() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试字符串运算
    let script = r#"
    $s1 = "Hello"
    $s2 = "World"
    $result = $s1 + " " + $s2
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::String("Hello World".to_string()));
}

#[test]
fn test_array_operations() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试数组运算
    let script = r#"
    $arr = [1, 2, 3]
    $result = $arr.length
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(3));
}

#[test]
fn test_complex_arithmetic() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试复杂算术运算
    let script = r#"
    $result = 1 + 2 * 3 - 4 / 2
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(5));
}

#[test]
fn test_comparison_operations() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试比较运算
    let script = r#"
    $a = 5
    $b = 10
    $result = $a < $b
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Boolean(true));
}

#[test]
fn test_nil_handling() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试 nil 处理
    let script = r#"
    $result = nil
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Nil);
}

#[test]
fn test_float_operations() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试浮点数运算
    let script = r#"
    $result = 1.5 + 2.5
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Float(4.0));
}

#[test]
fn test_symbol_handling() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试符号处理
    let script = r#"
    $result = :test
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Symbol("test".to_string()));
}

#[test]
fn test_if_statement() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试 if 语句
    let script = r#"
    $x = 10
    if $x > 5
        $result = "Greater than 5"
    else
        $result = "Less than or equal to 5"
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::String("Greater than 5".to_string()));
}

#[test]
fn test_while_loop() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试 while 循环
    let script = r#"
    $i = 0
    $result = 0
    while $i < 5
        $result += $i
        $i += 1
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(10));
}

#[test]
fn test_for_loop() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试 for 循环
    let script = r#"
    $arr = [1, 2, 3, 4, 5]
    $result = 0
    for $i in $arr
        $result += $i
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(15));
}

#[test]
fn test_break_statement() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试 break 语句
    let script = r#"
    $i = 0
    $result = 0
    while $i < 10
        if $i == 5
            break
        end
        $result += $i
        $i += 1
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(10));
}

#[test]
fn test_until_loop() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试 until 循环
    let script = r#"
    $i = 0
    $result = 0
    until $i >= 5
        $result += $i
        $i += 1
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(10));
}

#[test]
fn test_case_statement() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试 case 语句
    let script = r#"
    $x = 2
    case $x
    when 1
        $result = "One"
    when 2
        $result = "Two"
    when 3
        $result = "Three"
    else
        $result = "Other"
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::String("Two".to_string()));
}

#[test]
fn test_next_statement() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试 next 语句
    let script = r#"
    $i = 0
    $result = 0
    while $i < 5
        $i += 1
        if $i == 3
            next
        end
        $result += $i
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(7));
}

#[test]
fn test_redo_statement() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试 redo 语句
    let script = r#"
    $i = 0
    $result = 0
    while $i < 3
        $i += 1
        if $i == 2
            $result += 1
            redo
        end
        $result += $i
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(8));
}

#[test]
fn test_hash_operations() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试哈希操作
    let script = r#"
    $hash = { "name" => "John", "age" => 30, "city" => "New York" }
    $result = $hash["name"]
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::String("John".to_string()));
}

#[test]
fn test_symbol_operations() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试符号操作
    let script = r#"
    $symbol1 = :test
    $symbol2 = :test
    $result = $symbol1 == $symbol2
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Boolean(true));
}
