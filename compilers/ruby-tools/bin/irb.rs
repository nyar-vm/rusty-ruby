//! IRB (Interactive Ruby) 工具
//! 提供交互式 Ruby 解释器功能

use ruby_tools::RustyRubyFrontend;
use std::io::{self, BufRead, Write};

/// 显示版本信息
fn print_version() {
    println!("irb 1.11.0 (2024-12-25)");
}

/// 显示帮助信息
fn print_help() {
    println!("Usage: irb [switches] [programfile] [arguments]");
    println!("  -f                    Suppress read of ~/.irbrc");
    println!("  -m                    Bc mode (load mathn, fraction or matrix)");
    println!("  -d                    Set $DEBUG to true (same as `ruby -d')");
    println!("  -r library            Require the library, before executing your script");
    println!("  -I path               Specify $LOAD_PATH directory (may be used multiple times)");
    println!("  -U                    Set default encoding to UTF-8");
    println!("  --version             Print the version of irb");
    println!("  --help                Print this help");
}

/// 启动交互式会话
fn start_interactive_session() {
    println!("irb 1.11.0 (2024-12-25) -- Interactive Ruby");
    println!("Type 'exit' or 'quit' to exit.");
    println!();

    let frontend = RustyRubyFrontend::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut line_number = 1;

    loop {
        print!("irb(main):{:03}:0>", line_number);
        stdout.flush().unwrap();

        match stdin.lock().read_line(&mut input) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let trimmed_input = input.trim();
                if trimmed_input == "exit" || trimmed_input == "quit" {
                    break;
                }
                if !trimmed_input.is_empty() {
                    match frontend.parse(trimmed_input) {
                        Ok(ast) => {
                            println!("=> {:?}", ast);
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                }
                input.clear();
                line_number += 1;
            }
            Err(e) => {
                println!("Error reading input: {:?}", e);
                break;
            }
        }
    }

    println!("Exiting irb...");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

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
            _ if arg.starts_with('-') => {
                // 处理其他参数
                println!("Warning: Option {} not yet implemented", arg);
            }
            _ => {
                // 执行脚本文件
                println!("Error: irb does not support script files");
                return;
            }
        }
    }

    // 启动交互式会话
    start_interactive_session();
}
