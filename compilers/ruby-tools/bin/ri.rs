//! RI (Ruby Information) 文档查看器
//! 在终端中查看 Ruby 类、方法等的文档

use std::env;

/// 显示版本信息
fn print_version() {
    println!("ri 13.1.0");
}

/// 显示帮助信息
fn print_help() {
    println!("Usage: ri [options] [name]");
    println!("");
    println!("Options:");
    println!("  -d, --directory=DIRECTORY     List of directories to search for documentation");
    println!("  -f, --format=FORMAT           Set output format (ansi, bs, html, plain, simple)");
    println!("  -h, --help                    Show this help message and exit");
    println!("  -T, --no-pager                Send output directly to stdout, without a pager");
    println!("  -v, --verbose                 Display extra information");
    println!("  -V, --version                 Display version information and exit");
    println!("  --system                     List only documentation from the standard library");
    println!("  --site                       List only documentation from site_ruby");
    println!("  --gems                       List only documentation from gems");
    println!("  --all                        List documentation from all sources");
    println!("  --no-standard-docs           Do not include documentation from the standard library");
    println!("  --no-site-docs               Do not include documentation from site_ruby");
    println!("  --no-gems-docs               Do not include documentation from gems");
    println!("  --line-numbers               Show line numbers in source code");
    println!("  --no-line-numbers            Do not show line numbers in source code");
    println!("  --show-all                   Show all documentation for a class or module");
    println!("  --no-show-all                Do not show all documentation for a class or module");
    println!("  --use-system-locale          Use the system locale for output");
    println!("  --no-use-system-locale       Do not use the system locale for output");
}

/// 查找文档
fn lookup_documentation(query: &str) {
    println!("Looking up documentation for: {}", query);

    // 模拟文档查找逻辑
    match query {
        "String" => {
            println!("\n= String");
            println!("\nRuby's String class represents character strings.");
            println!("\nMethods:");
            println!("  length() - Returns the length of the string");
            println!("  empty?() - Returns true if the string is empty");
            println!("  upcase() - Returns a new string with all uppercase letters");
            println!("  downcase() - Returns a new string with all lowercase letters");
        }
        "String#length" => {
            println!("\n= String#length");
            println!("\nReturns the length of the string.");
            println!("\nExamples:");
            println!("  'hello'.length  # => 5");
            println!("  ''.length       # => 0");
        }
        "Array" => {
            println!("\n= Array");
            println!("\nRuby's Array class represents ordered, integer-indexed collections of any object.");
            println!("\nMethods:");
            println!("  length() - Returns the number of elements in the array");
            println!("  push() - Appends an object to the end of the array");
            println!("  pop() - Removes and returns the last element of the array");
            println!("  each() - Calls the given block once for each element");
        }
        "Array#each" => {
            println!("\n= Array#each");
            println!("\nCalls the given block once for each element in self.");
            println!("\nExamples:");
            println!("  [1, 2, 3].each {{ |i| puts i }}  # prints 1, 2, 3");
        }
        _ => {
            println!("Documentation not found for: {}", query);
            println!("Try another query or run 'ri' for usage.");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 处理命令行参数
    let mut query = String::new();

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
                query.push_str(arg);
                query.push(' ');
            }
        }
    }

    let query = query.trim();

    if query.is_empty() {
        print_help();
        return;
    }

    lookup_documentation(query);
}
