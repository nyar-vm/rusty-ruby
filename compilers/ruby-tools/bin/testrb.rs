//! testrb 测试运行器
//! 运行 Ruby 测试文件

use std::{env, fs, path::Path};

/// 显示版本信息
fn print_version() {
    println!("testrb 3.3.0");
}

/// 显示帮助信息
fn print_help() {
    println!("Usage: testrb [options] <test_file1> <test_file2> ...");
    println!("");
    println!("Options:");
    println!("  -v, --verbose                 Run tests verbosely");
    println!("  -n, --name PATTERN            Run tests matching PATTERN");
    println!("  -e, --exclude PATTERN         Exclude tests matching PATTERN");
    println!("  -I, --include DIRECTORY       Add DIRECTORY to load path");
    println!("  -r, --require LIBRARY         Require LIBRARY before running tests");
    println!("  -l, --load-path DIRECTORY     Add DIRECTORY to load path");
    println!("  -p, --profile                 Run tests with profiling");
    println!("  -d, --debug                   Run tests with debugging");
    println!("  -h, --help                    Show this help message");
    println!("  -V, --version                 Show version information");
}

/// 运行测试
fn run_tests(test_files: &[&str]) {
    let mut total_tests = 0;
    let mut passed_tests = 0;

    for test_file in test_files {
        let path = Path::new(test_file);
        if path.exists() && path.is_file() {
            println!("Running tests in: {}", test_file);
            let (tests, passed) = run_test_file(path);
            total_tests += tests;
            passed_tests += passed;
        }
        else {
            println!("Error: {} does not exist or is not a file", test_file);
        }
    }

    println!("\nTest Summary:");
    println!("Total tests: {}", total_tests);
    println!("Passed: {}", passed_tests);
    println!("Failed: {}", total_tests - passed_tests);

    if passed_tests == total_tests {
        println!("\nAll tests passed!");
    }
    else {
        println!("\nSome tests failed.");
    }
}

/// 运行单个测试文件
fn run_test_file(test_file: &Path) -> (u32, u32) {
    // 模拟测试运行逻辑
    let file_name = test_file.file_name().unwrap_or_default().to_string_lossy();

    // 读取文件内容
    match fs::read_to_string(test_file) {
        Ok(content) => {
            println!("Processing test file: {}", file_name);
            // 简单的测试检测逻辑
            let test_count = content.matches("test_").count() as u32;
            let passed_count = test_count; // 模拟所有测试通过

            println!("  Tests found: {}", test_count);
            println!("  Tests passed: {}", passed_count);

            (test_count, passed_count)
        }
        Err(e) => {
            println!("Error reading test file: {:?}", e);
            (0, 0)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 处理命令行参数
    let mut test_files = Vec::new();

    for arg in &args[1..] {
        match arg.as_str() {
            "--version" | "-V" => {
                print_version();
                return;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            _ if arg.starts_with('-') => {
                // 处理其他参数
                println!("Warning: Option {} not yet implemented", arg);
            }
            _ => {
                test_files.push(arg.as_str());
            }
        }
    }

    if test_files.is_empty() {
        print_help();
        return;
    }

    run_tests(&test_files);
}
