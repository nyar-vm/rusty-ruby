//! FFI 测试

use ruby::{Ruby, ffi};

#[test]
fn test_ffi_basic() {
    // 创建 Ruby 运行时环境
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 加载标准 C 库（Windows 上为 msvcrt.dll，Linux 上为 libc.so.6）
    #[cfg(windows)]
    let lib_path = "msvcrt.dll";
    #[cfg(unix)]
    let lib_path = "libc.so.6";

    // 加载库
    let load_result = ffi::load_library(&mut ruby, lib_path);
    assert!(load_result.is_ok(), "Failed to load library: {:?}", load_result);
    assert!(load_result.unwrap(), "Failed to load library");

    println!("Library loaded successfully");

    // 尝试获取一个简单的函数（这里我们尝试获取 rand 函数）
    let func_key_result = ffi::get_function(&mut ruby, lib_path, "rand");
    assert!(func_key_result.is_ok(), "Failed to get function: {:?}", func_key_result);
    let func_key = func_key_result.unwrap();

    println!("Function obtained successfully: {}", func_key);

    // 调用函数
    let call_result = ffi::call_function(&mut ruby, &func_key);
    assert!(call_result.is_ok(), "Failed to call function: {:?}", call_result);
    let result = call_result.unwrap();

    println!("Function called successfully, result: {}", result);
    assert!(result >= 0, "Invalid result from rand function");
}

#[test]
fn test_ffi_error_handling() {
    // 创建 Ruby 运行时环境
    let mut ruby = Ruby::new().expect("Failed to create Ruby runtime");

    // 尝试加载不存在的库
    let load_result = ffi::load_library(&mut ruby, "nonexistent_library.dll");
    assert!(load_result.is_err(), "Should fail to load nonexistent library");

    // 尝试获取不存在的函数
    let func_key_result = ffi::get_function(&mut ruby, "nonexistent_library.dll", "nonexistent_function");
    assert!(func_key_result.is_err(), "Should fail to get function from nonexistent library");

    // 尝试调用不存在的函数
    let call_result = ffi::call_function(&mut ruby, "nonexistent_function");
    assert!(call_result.is_err(), "Should fail to call nonexistent function");
}
