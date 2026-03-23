//! FFI 示例
//! 
//! 这个示例展示了如何使用 Rusty Ruby 的 FFI 功能来调用 C 函数

use ruby::{Ruby, ffi};

fn main() {
    println!("=== Rusty Ruby FFI 示例 ===");
    
    // 创建 Ruby 运行时环境
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");
    
    // 加载标准 C 库
    #[cfg(windows)]
    let lib_path = "msvcrt.dll";
    #[cfg(unix)]
    let lib_path = "libc.so.6";
    
    println!("正在加载库: {}", lib_path);
    match ffi::load_library(&mut ruby, lib_path) {
        Ok(true) => println!("库加载成功！"),
        Ok(false) => {
            println!("库加载失败！");
            return;
        }
        Err(err) => {
            println!("库加载错误: {:?}", err);
            return;
        }
    }
    
    // 获取 rand 函数
    println!("正在获取 rand 函数...");
    match ffi::get_function(&mut ruby, lib_path, "rand") {
        Ok(func_key) => {
            println!("函数获取成功，键: {}", func_key);
            
            // 调用 rand 函数
            println!("正在调用 rand 函数...");
            match ffi::call_function(&mut ruby, &func_key) {
                Ok(result) => println!("rand() 返回值: {}", result),
                Err(err) => println!("调用函数错误: {:?}", err),
            }
            
            // 多次调用以展示随机性
            println!("\n多次调用 rand 函数:");
            for i in 0..5 {
                match ffi::call_function(&mut ruby, &func_key) {
                    Ok(result) => println!("rand() #{i+1}: {}", result),
                    Err(err) => println!("调用函数错误: {:?}", err),
                }
            }
        }
        Err(err) => println!("获取函数错误: {:?}", err),
    }
    
    println!("\n=== 示例结束 ===");
}
