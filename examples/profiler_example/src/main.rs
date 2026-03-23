//! 性能分析和调试工具示例
//! 
//! 展示如何使用 Rusty Ruby 的性能分析和调试工具。

use ruby::Ruby;

fn main() -> ruby::Result<()> {
    println!("=== Rusty Ruby 性能分析和调试工具示例 ===");
    println!();

    // 创建 Ruby 运行时实例
    let mut ruby = Ruby::new()?;
    
    // 获取执行上下文
    let context = ruby.vm().context();
    let mut context = context.lock().unwrap();
    
    println!("1. 启用性能分析");
    // 启用性能分析
    context.enable_profiling();
    println!("性能分析已启用");
    println!();

    // 执行一些 Ruby 代码
    println!("2. 执行 Ruby 代码");
    println!("执行脚本: $result = 1 + 2 * 3");
    ruby.execute_script("$result = 1 + 2 * 3")?;
    
    println!("执行脚本: $s1 = \"Hello\"; $s2 = \"World\"; $result = $s1 + \" \" + $s2");
    ruby.execute_script("$s1 = \"Hello\"; $s2 = \"World\"; $result = $s1 + \" \" + $s2")?;
    
    println!("执行脚本: $arr = [1, 2, 3, 4, 5]; $result = $arr.length");
    ruby.execute_script("$arr = [1, 2, 3, 4, 5]; $result = $arr.length")?;
    println!();

    // 生成性能分析报告
    println!("3. 生成性能分析报告");
    let report = context.generate_profile_report();
    println!("{}", report);

    // 生成可视化报告
    println!("4. 生成可视化报告");
    let visualization_report = context.generate_visualization_report();
    println!("{}", visualization_report);

    // 禁用性能分析
    println!("5. 禁用性能分析");
    context.disable_profiling();
    println!("性能分析已禁用");
    println!();

    println!("=== 示例完成 ===");
    
    Ok(())
}
