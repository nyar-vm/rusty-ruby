//! JIT 编译器测试

use ruby::Ruby;

#[test]
fn test_jit_compilation() {
    let mut ruby = Ruby::new().unwrap();

    // 执行一个简单的计算，循环多次以触发JIT编译
    let script = r#"
        sum = 0
        for i in 0..100
            sum += i
        end
        sum
    "#;

    // 多次执行同一脚本，以触发JIT编译
    for i in 0..20 {
        println!("Executing script #{}", i + 1);
        let result = ruby.execute_script(script);
        assert!(result.is_ok());

        // 获取并打印结果
        let sum = ruby.get_global("sum").unwrap();
        println!("Result: {:?}", sum);
    }

    println!("JIT test completed successfully");
}

#[test]
fn test_jit_arithmetic() {
    let mut ruby = Ruby::new().unwrap();

    // 测试算术操作的JIT编译
    let script = r#"
        a = 10
        b = 20
        c = a + b * 2 - 5
        c
    "#;

    // 多次执行以触发JIT
    for i in 0..15 {
        println!("Executing arithmetic script #{}", i + 1);
        let result = ruby.execute_script(script);
        assert!(result.is_ok());

        let c = ruby.get_global("c").unwrap();
        println!("Arithmetic result: {:?}", c);
    }

    println!("JIT arithmetic test completed successfully");
}
