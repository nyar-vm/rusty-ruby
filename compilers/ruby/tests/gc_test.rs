//! 垃圾收集器测试

use ruby::Ruby;

#[test]
fn test_gc_basic() {
    // 创建Ruby运行时
    let mut ruby = Ruby::new().unwrap();

    // 执行一个创建大量对象的脚本
    let script = r#"
    # 创建大量数组
    array = []
    1000.times do |i|
        array << i
    end
    
    # 创建大量哈希
    hash = {}
    1000.times do |i|
        hash[i] = i * 2
    end
    
    # 打印结果
    puts "Array length: #{array.length}"
    puts "Hash size: #{hash.size}"
    "#;

    let result = ruby.execute_script(script);
    assert!(result.is_ok(), "Script execution failed: {:?}", result);

    println!("GC test completed");
}

#[test]
fn test_gc_object_lifetime() {
    // 创建Ruby运行时
    let mut ruby = Ruby::new().unwrap();

    // 执行脚本，创建对象后释放引用
    let script = r#"
    # 创建对象
    obj = { :name => "test", :value => 42 }
    puts "Created object: #{obj}"
    
    # 释放引用
    obj = nil
    puts "Set object to nil"
    "#;

    let result = ruby.execute_script(script);
    assert!(result.is_ok(), "Script execution failed: {:?}", result);

    println!("Object lifetime test completed");
}
