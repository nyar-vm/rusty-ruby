//! RDoc 文档生成器
//! 从 Ruby 代码中提取注释并生成 HTML 文档

use std::{env, fs, path::Path};

/// 显示版本信息
fn print_version() {
    println!("rdoc 6.6.3");
}

/// 显示帮助信息
fn print_help() {
    println!("Usage: rdoc [options] [names...]");
    println!("");
    println!("Options:");
    println!("  -a, --all                      Include all files");
    println!("  -B, --build-list                Build class and method list");
    println!("  -c, --charset=CHARSET           Charset for output files");
    println!("  -C, --[no-]coverage             Report code coverage");
    println!("  -d, --diagram                   Generate diagrams");
    println!("  -D, --[no-]dark                 Use a dark theme");
    println!("  -e, --encoding=ENCODING         Source file encoding");
    println!("  -f, --format=FORMAT             Set the output formatter");
    println!("  -F, --files                     Show list of files to be processed");
    println!("  -h, --help                      Show this help");
    println!("  -i, --include=DIRECTORY         Add DIRECTORY to load path");
    println!("  -I, --input-format=NAME         Set the input format");
    println!("  -l, --line-numbers              Show line numbers in source code");
    println!("  -m, --main=NAME                 Specify the main class/file");
    println!("  -M, --markup=NAME               Set the markup language");
    println!("  -n, --no-private                Exclude private methods");
    println!("  -N, --line-number=RANGE         Show line numbers in source code");
    println!("  -o, --op=DIRECTORY              Set the output directory");
    println!("  -P, --[no-]promiscuous          Require any file");
    println!("  -p, --page-dir=DIRECTORY        Template directory");
    println!("  -Q, --[no-]quiet                Quiet output");
    println!("  -r, --require=FEATURE           Require a feature");
    println!("  -s, --[no-]strip                Strip hashes from headings");
    println!("  -t, --title=TITLE               Set the title");
    println!("  -T, --template=NAME             Set the template");
    println!("  -U, --[no-]webcrawler           Do not follow hyperlinks");
    println!("  -v, --verbose                   Display extra information");
    println!("  -V, --version                   Display version");
    println!("  -x, --exclude=PATTERN           Exclude files matching PATTERN");
    println!("  -y, --[no-]yard-compatibility   YARD compatibility mode");
    println!("  -Z, --[no-]zen                  Zen mode (minimal output)");
}

/// 生成文档
fn generate_docs(paths: &[&str]) {
    let output_dir = Path::new("doc");

    // 创建输出目录
    if !output_dir.exists() {
        match fs::create_dir_all(output_dir) {
            Ok(_) => println!("Created output directory: doc/"),
            Err(e) => {
                println!("Error creating output directory: {:?}", e);
                return;
            }
        }
    }

    // 处理每个路径
    for path in paths {
        let path_obj = Path::new(path);
        if path_obj.is_file() {
            if path_obj.extension().map_or(false, |ext| ext == "rb") {
                println!("Processing file: {}", path_obj.display());
                // 这里应该实现实际的文件处理逻辑
                // 提取注释并生成文档
            }
        }
        else if path_obj.is_dir() {
            println!("Processing directory: {}", path_obj.display());
            // 这里应该实现实际的目录处理逻辑
        }
        else {
            println!("Warning: {} is not a file or directory", path);
        }
    }

    println!("Documentation generated in doc/ directory");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 处理命令行参数
    let mut paths = Vec::new();

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
                paths.push(arg.as_str());
            }
        }
    }

    if paths.is_empty() {
        println!("Generating documentation for current directory...");
        generate_docs(&["."]);
    }
    else {
        println!("Generating documentation for specified paths...");
        generate_docs(&paths);
    }
}
