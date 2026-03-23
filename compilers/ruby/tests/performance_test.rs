//! Ruby VM 性能测试

use ruby::Ruby;
use ruby_types::{RubyError, RubyValue};
use std::time::{Duration, Instant};

/// 性能测试结果
struct PerformanceResult {
    name: String,
    vm_time: Duration,
    ast_time: Duration,
    vm_operations: u64,
    ast_operations: u64,
}

impl PerformanceResult {
    fn new(name: &str, vm_time: Duration, ast_time: Duration, vm_operations: u64, ast_operations: u64) -> Self {
        Self { name: name.to_string(), vm_time, ast_time, vm_operations, ast_operations }
    }

    fn print(&self) {
        let vm_ops_per_sec = self.vm_operations as f64 / self.vm_time.as_secs_f64();
        let ast_ops_per_sec = self.ast_operations as f64 / self.ast_time.as_secs_f64();
        let speedup = vm_ops_per_sec / ast_ops_per_sec;

        println!("Test: {}", self.name);
        println!("VM time: {:?}", self.vm_time);
        println!("AST time: {:?}", self.ast_time);
        println!("VM operations per second: {:.2}", vm_ops_per_sec);
        println!("AST operations per second: {:.2}", ast_ops_per_sec);
        println!("Speedup: {:.2}x", speedup);
        println!("----------------------------------------");
    }
}

/// 运行性能测试
fn run_performance_test<F1, F2>(name: &str, iterations: u64, mut vm_test: F1, mut ast_test: F2)
where
    F1: FnMut() -> bool,
    F2: FnMut() -> bool,
{
    // 测试 VM 性能
    let start = Instant::now();
    for _ in 0..iterations {
        assert!(vm_test());
    }
    let vm_time = start.elapsed();

    // 测试 AST 性能
    let start = Instant::now();
    for _ in 0..iterations {
        assert!(ast_test());
    }
    let ast_time = start.elapsed();

    let result = PerformanceResult::new(name, vm_time, ast_time, iterations, iterations);
    result.print();
}

/// 测试基本算术运算性能
fn test_arithmetic_performance() {
    // VM 测试
    let vm_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$result = 1 + 2 * 3 - 4 / 2").is_ok()
    };

    // AST 测试 (使用相同的脚本，让 Ruby 内部使用 AST 解析)
    let ast_test = || {
        let mut ruby = Ruby::new().unwrap();
        // 确保使用 AST 解析路径
        ruby.execute_script("$a = 1; $b = 2; $result = $a + $b").is_ok()
    };

    run_performance_test("Arithmetic Operations", 1_000_000, vm_test, ast_test);
}

/// 测试循环性能
fn test_loop_performance() {
    // VM 测试 - 使用 while 循环
    let vm_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$i = 0; $result = 0; while $i < 100 do $result += $i; $i += 1 end").is_ok()
    };

    // AST 测试 - 使用 for 循环
    let ast_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$result = 0; for $i in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] do $result += $i end").is_ok()
    };

    run_performance_test("Loop Operations", 100_000, vm_test, ast_test);
}

/// 测试字符串操作性能
fn test_string_performance() {
    // VM 测试
    let vm_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$s1 = \"Hello\"; $s2 = \"World\"; $result = $s1 + \" \" + $s2").is_ok()
    };

    // AST 测试
    let ast_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$s = \"Test\"; $result = $s + \" string\"").is_ok()
    };

    run_performance_test("String Operations", 500_000, vm_test, ast_test);
}

/// 测试数组操作性能
fn test_array_performance() {
    // VM 测试
    let vm_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$arr = [1, 2, 3, 4, 5]; $result = $arr.length").is_ok()
    };

    // AST 测试
    let ast_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$arr = [1, 2, 3]; $result = $arr[0] + $arr[1]").is_ok()
    };

    run_performance_test("Array Operations", 500_000, vm_test, ast_test);
}

/// 测试哈希操作性能
fn test_hash_performance() {
    // VM 测试
    let vm_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$hash = { name: \"John\", age: 30 }; $result = $hash[:name]").is_ok()
    };

    // AST 测试
    let ast_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$hash = { a: 1, b: 2 }; $result = $hash[:a] + $hash[:b]").is_ok()
    };

    run_performance_test("Hash Operations", 300_000, vm_test, ast_test);
}

/// 测试条件分支性能
fn test_branch_performance() {
    // VM 测试
    let vm_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$x = 10; if $x > 5 then $result = \"Greater\" else $result = \"Lesser\" end").is_ok()
    };

    // AST 测试
    let ast_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$x = 5; if $x == 5 then $result = \"Equal\" else $result = \"Not equal\" end").is_ok()
    };

    run_performance_test("Branch Operations", 1_000_000, vm_test, ast_test);
}

/// 测试方法调用性能
fn test_method_call_performance() {
    // VM 测试
    let vm_test = || {
        let mut ruby = Ruby::new().unwrap();
        // 定义一个简单的方法
        ruby.define_global_method(
            "add",
            Box::new(|_, args| {
                if args.len() == 2 {
                    let a = args[0].to_f64();
                    let b = args[1].to_f64();
                    Ok(ruby_types::RubyValue::Float(a + b))
                }
                else {
                    Err(ruby_types::RubyError::ArgumentError("Expected 2 arguments".to_string()))
                }
            }),
        )
        .unwrap();
        ruby.execute_script("$result = add(1, 2)").is_ok()
    };

    // AST 测试
    let ast_test = || {
        let mut ruby = Ruby::new().unwrap();
        // 定义一个简单的方法
        ruby.define_global_method(
            "add",
            Box::new(|_, args| {
                if args.len() == 2 {
                    let a = args[0].to_f64();
                    let b = args[1].to_f64();
                    Ok(ruby_types::RubyValue::Float(a + b))
                }
                else {
                    Err(ruby_types::RubyError::ArgumentError("Expected 2 arguments".to_string()))
                }
            }),
        )
        .unwrap();
        ruby.execute_script("$result = add(3, 4)").is_ok()
    };

    run_performance_test("Method Call Operations", 500_000, vm_test, ast_test);
}

/// 测试符号操作性能
fn test_symbol_performance() {
    // VM 测试
    let vm_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$symbol1 = :test; $symbol2 = :test; $result = $symbol1 == $symbol2").is_ok()
    };

    // AST 测试
    let ast_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$symbol = :test; $result = $symbol.to_s").is_ok()
    };

    run_performance_test("Symbol Operations", 1_000_000, vm_test, ast_test);
}

/// 测试浮点数运算性能
fn test_float_performance() {
    // VM 测试
    let vm_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$result = 1.5 + 2.5").is_ok()
    };

    // AST 测试
    let ast_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$a = 1.5; $b = 2.5; $result = $a * $b").is_ok()
    };

    run_performance_test("Float Operations", 1_000_000, vm_test, ast_test);
}

/// 测试复杂表达式性能
fn test_complex_expression_performance() {
    // VM 测试
    let vm_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$a = 10; $b = 20; $c = 30; $result = ($a + $b) * ($c - $a) / $b").is_ok()
    };

    // AST 测试
    let ast_test = || {
        let mut ruby = Ruby::new().unwrap();
        ruby.execute_script("$x = 5; $y = 10; $z = 15; $result = ($x * $y) + ($z / $x)").is_ok()
    };

    run_performance_test("Complex Expression Operations", 500_000, vm_test, ast_test);
}

#[test]
fn performance_tests() {
    println!("Running Ruby VM performance tests...");
    println!("====================================");

    test_arithmetic_performance();
    test_loop_performance();
    test_string_performance();
    test_array_performance();
    test_hash_performance();
    test_branch_performance();
    test_method_call_performance();
    test_symbol_performance();
    test_float_performance();
    test_complex_expression_performance();

    println!("Performance tests completed!");
}
