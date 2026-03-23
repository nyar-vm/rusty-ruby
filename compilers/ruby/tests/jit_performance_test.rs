//! JIT 性能测试

use ruby::{Instruction, VM};
use ruby_types::RubyValue;
use std::time::Instant;

#[test]
pub fn test_jit_performance() {
    // 创建虚拟机
    let vm = VM::new();

    // 生成一个计算斐波那契数列的指令序列
    // 这个序列会被执行多次，触发JIT编译
    let instructions = generate_fibonacci_instructions(30);

    // 执行多次，观察性能变化
    let mut times = Vec::new();

    for i in 0..20 {
        let start = Instant::now();

        // 执行指令
        let result = vm.execute(&instructions);
        assert!(result.is_ok());

        let duration = start.elapsed();
        times.push(duration.as_millis());
        println!("Execution {}: {}ms", i + 1, duration.as_millis());
    }

    // 分析性能数据
    let average = times.iter().sum::<u128>() / times.len() as u128;
    let min = times.iter().min().unwrap();
    let max = times.iter().max().unwrap();

    println!("\nPerformance analysis:");
    println!("Average: {}ms", average);
    println!("Minimum: {}ms", min);
    println!("Maximum: {}ms", max);

    // 验证JIT编译是否提高了性能
    // 通常情况下，后面的执行应该比前面的快
    let first_half_average = times[0..10].iter().sum::<u128>() / 10;
    let second_half_average = times[10..20].iter().sum::<u128>() / 10;

    println!("\nFirst half average: {}ms", first_half_average);
    println!("Second half average: {}ms", second_half_average);

    // 检查性能是否有提升
    assert!(second_half_average <= first_half_average, "JIT compilation did not improve performance");
}

/// 生成计算斐波那契数列的指令序列
fn generate_fibonacci_instructions(n: u32) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    // 加载常量 n
    instructions.push(Instruction::LoadConst(RubyValue::Integer(n as i32)));

    // 这里简化处理，实际的斐波那契计算需要更复杂的指令序列
    // 我们使用一个简单的循环来模拟计算过程
    for _ in 0..1000 {
        instructions.push(Instruction::LoadConst(RubyValue::Integer(1)));
        instructions.push(Instruction::LoadConst(RubyValue::Integer(1)));
        instructions.push(Instruction::Add);
        instructions.push(Instruction::LoadConst(RubyValue::Integer(1)));
        instructions.push(Instruction::Mul);
    }

    instructions
}
