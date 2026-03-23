//! Ruby 命令行工具
//! 运行 Ruby 脚本

use ruby_tools::RustyRubyFrontend;
use std::{env, fs};

/// 显示版本信息
fn print_version() {
    println!("ruby 3.3.0 (2024-12-25) [x86_64-pc-windows-msvc]");
}

/// 显示帮助信息
fn print_help() {
    println!("Usage: ruby [switches] [--] [programfile] [arguments]");
    println!("  -0[octal]       specify record separator (\0, if no argument)");
    println!("  -a              autosplit mode with -n or -p (splits $_ into $F)");
    println!("  -c              check syntax only");
    println!("  -Cdirectory     cd to directory, before executing your script");
    println!("  -d              set debugging flags (set $DEBUG to true)");
    println!("  -e 'command'    one line of script. Several -e's allowed. Omit [programfile]");
    println!("  -Eex[:in]       specify the default external and internal character encodings");
    println!("  -Fpattern       split() pattern for autosplit (-a)");
    println!("  -i[extension]   edit ARGV files in place (make backup if extension supplied)");
    println!("  -Idirectory     specify $LOAD_PATH directory (may be used multiple times)");
    println!("  -l              enable line ending processing");
    println!("  -n              assume 'while gets(); ... end' loop around your script");
    println!("  -p              assume loop like -n but print line also like sed");
    println!("  -rlibrary       require the library, before executing your script");
    println!("  -s              enable some switch parsing for switches after script name");
    println!("  -S              look for the script using PATH environment variable");
    println!("  -T[level]       turn on tainting checks");
    println!("  -v              print version number, then turn on verbose mode");
    println!("  -w              turn warnings on for your script");
    println!("  -W[level]       set warning level; 0=silence, 1=medium, 2=verbose");
    println!("  --copyright     print the copyright");
    println!("  --version       print the version number");
    println!("  --help          show this message");
}

/// 运行 Ruby 脚本
fn run_script(script_path: &str) {
    println!("Running Ruby script: {}", script_path);

    match fs::read_to_string(script_path) {
        Ok(content) => {
            let frontend = RustyRubyFrontend::new();
            match frontend.parse(&content) {
                Ok(ast) => {
                    println!("Script parsed successfully");
                    println!("AST: {:?}", ast);
                }
                Err(e) => {
                    println!("Error parsing script: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error reading script: {:?}", e);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 处理命令行参数
    for arg in &args[1..] {
        match arg.as_str() {
            "--version" => {
                print_version();
                return;
            }
            "--help" => {
                print_help();
                return;
            }
            "-v" => {
                print_version();
                // 后续实现 verbose 模式
                return;
            }
            _ if arg.starts_with('-') => {
                // 处理其他参数
                println!("Warning: Option {} not yet implemented", arg);
            }
            _ => {
                // 执行脚本文件
                run_script(arg);
                return;
            }
        }
    }

    // 没有参数时显示帮助信息
    print_help();
}
