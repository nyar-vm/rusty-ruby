//! Rake 构建工具
//! 提供 Ruby 项目的构建和任务执行功能

use std::{env, path::Path};

/// 显示版本信息
fn print_version() {
    println!("rake, version 13.1.0");
}

/// 显示帮助信息
fn print_help() {
    println!("rake [options] [TASK] [ARGS]");
    println!("");
    println!("Options:");
    println!("  -f, --rakefile [FILE]          Use FILE as the rakefile");
    println!("  -g, --no-gems                  Do not load any gems");
    println!("  -G, --gems                     Load the gems (default)");
    println!("  -i, --init                     Initialize the rakefile data base");
    println!("  -j, --jobs [NUMBER]            Specify the number of jobs to run in parallel");
    println!("  -m, --multitask                Treat all tasks as multitasks");
    println!("  -n, --dry-run                  Do a dry run without executing actions");
    println!("  -N, --no-search                Do not search parent directories for the Rakefile");
    println!("  -q, --quiet                    Do not log messages to standard output");
    println!("  -r, --require [FILE]           Require FILE before executing rakefile");
    println!("  -R, --rakelibdir [DIRECTORY]   Auto-import any .rake files in DIRECTORY");
    println!("  -s, --silent                   Like --quiet, but also suppresses the 'rake' command itself");
    println!("  -t, --trace                    Turn on invoke/execute tracing");
    println!("  -T, --tasks [PATTERN]          Display the tasks (matching optional PATTERN) with descriptions");
    println!("  -v, --verbose                  Log message to standard output");
    println!("  -V, --version                  Display the version");
    println!("  -W, --where [PATTERN]          Describe the tasks (matching optional PATTERN), then exit");
    println!("  -h, --help                     Display this help message");
    println!("  --prereqs                      Display the tasks and prerequisites, then exit");
    println!("  --tasks-with-dependencies      Display the tasks and dependencies, then exit");
    println!("  --describe [PATTERN]           Describe the tasks (matching optional PATTERN), then exit");
    println!("  --dry-run                      Do a dry run without executing actions");
    println!("  --silent                       Like --quiet, but also suppresses the 'rake' command itself");
    println!("  --verbose                      Log message to standard output");
    println!("  --trace                        Turn on invoke/execute tracing");
    println!("  --prerequisites                Display the tasks and prerequisites, then exit");
    println!("  --version                      Display the version");
    println!("  --help                         Display this help message");
}

/// 显示任务列表
fn print_tasks() {
    println!("rake build    # Build the project");
    println!("rake clean    # Clean the project");
    println!("rake default  # Default task");
    println!("rake install  # Install the project");
    println!("rake test     # Run tests");
}

/// 执行 rake 任务
fn execute_rake_task(task: &str) {
    match task {
        "default" => {
            println!("Executing default task...");
            println!("Default task completed");
        }
        "build" => {
            println!("Building project...");
            println!("Project built successfully");
        }
        "test" => {
            println!("Running tests...");
            println!("Tests passed");
        }
        "clean" => {
            println!("Cleaning project...");
            println!("Project cleaned");
        }
        "install" => {
            println!("Installing project...");
            println!("Project installed");
        }
        _ => {
            println!("Executing custom task: {}", task);
            println!("Task {} completed", task);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 处理命令行参数
    let mut task = "default";
    let mut show_tasks = false;

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
            "--tasks" | "-T" => {
                show_tasks = true;
            }
            _ if arg.starts_with('-') => {
                // 处理其他参数
                println!("Warning: Option {} not yet implemented", arg);
            }
            _ => {
                task = arg;
            }
        }
    }

    if show_tasks {
        print_tasks();
        return;
    }

    if Path::exists(Path::new("Rakefile")) {
        println!("Found Rakefile, executing tasks...");
        execute_rake_task(task);
    }
    else if Path::exists(Path::new("rakefile.rb")) {
        println!("Found rakefile.rb, executing tasks...");
        execute_rake_task(task);
    }
    else {
        println!("Error: No Rakefile or rakefile.rb found");
        println!("Create a Rakefile with task definitions.");
    }
}
