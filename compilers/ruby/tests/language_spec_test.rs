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

#[test]
fn test_method_definition() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试方法定义和调用
    let script = r#"
    def add(a, b)
        return a + b
    end
    
    $result = add(5, 3)
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(8));
}

#[test]
fn test_class_definition() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试类定义和方法调用
    let script = r#"
    class Person
        def initialize(name, age)
            @name = name
            @age = age
        end
        
        def greet
            return "Hello, my name is #{@name}"
        end
    end
    
    $person = Person.new("John", 30)
    $result = $person.greet
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::String("Hello, my name is John".to_string()));
}

#[test]
fn test_class_inheritance() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试类继承
    let script = r#"
    class Animal
        def speak
            return "Animal speaks"
        end
    end
    
    class Dog < Animal
        def speak
            return "Dog barks"
        end
    end
    
    $dog = Dog.new
    $result = $dog.speak
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::String("Dog barks".to_string()));
}

#[test]
fn test_module_include() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试模块包含
    let script = r#"
    module Greeting
        def greet
            return "Hello from module"
        end
    end
    
    class Person
        include Greeting
    end
    
    $person = Person.new
    $result = $person.greet
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::String("Hello from module".to_string()));
}

#[test]
fn test_block_usage() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试块的使用
    let script = r#"
    def with_block
        yield if block_given?
    end
    
    $result = ""
    with_block do
        $result = "Block executed"
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::String("Block executed".to_string()));
}

#[test]
fn test_exception_handling() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试异常处理
    let script = r#"
    begin
        raise "Test error"
    rescue => e
        $result = e.message
    end
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::String("Test error".to_string()));
}

#[test]
fn test_regex_operations() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试正则表达式操作
    let script = r#"
    $string = "Hello, world!"
    $result = $string =~ /world/
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(7));
}

#[test]
fn test_range_operations() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试范围操作
    let script = r#"
    $range = 1..5
    $result = $range.to_a.length
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::Integer(5));
}

#[test]
fn test_string_interpolation() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试字符串插值
    let script = r#"
    $name = "John"
    $age = 30
    $result = "Hello, #{@name} is #{@age} years old"
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    assert_eq!(result, RubyValue::String("Hello,  is  years old".to_string()));
}

#[test]
fn test_array_methods() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试数组方法
    let script = r#"
    $arr = [1, 2, 3, 4, 5]
    $result = $arr.map { |x| x * 2 }
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    // 这里需要根据实际的数组实现来断言
}

#[test]
fn test_hash_methods() {
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 测试哈希方法
    let script = r#"
    $hash = { "name" => "John", "age" => 30 }
    $result = $hash.keys
    puts $result
    "#;

    ruby.execute_script(script).unwrap();

    // 检查结果是否正确
    let result = ruby.get_global("$result").unwrap();
    // 这里需要根据实际的哈希实现来断言
}
