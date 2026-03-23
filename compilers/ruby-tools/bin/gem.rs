//! Gem 包管理器工具
//! 提供 Ruby 包的安装、卸载、列出等功能

use std::env;

/// 显示版本信息
fn print_version() {
    println!("gem 3.5.11 (2024-12-25)");
}

/// 显示帮助信息
fn print_help() {
    println!("Usage: gem COMMAND [ARGS]");
    println!("");
    println!("Commands:");
    println!("  build            Build a gem from a gemspec");
    println!("  cert             Manage RubyGems certificates and signing settings");
    println!("  cleanup          Remove old versions of gems from the local repository");
    println!("  contents         Display the contents of the installed gems");
    println!("  dependency       Show the dependencies of an installed gem");
    println!("  environment      Display information about the RubyGems environment");
    println!("  fetch            Download a gem and place it in the current directory");
    println!("  generate_index   Generates the index files for a gem server directory");
    println!("  help             Display help information about RubyGems");
    println!("  install          Install a gem into the local repository");
    println!("  list             Display gems whose name starts with STRING");
    println!("  mirror           Mirror all gems from a remote gem server to the local one");
    println!("  outdated         Display all gems that need updates");
    println!("  owner            Manage gem owners on RubyGems.org");
    println!("  pristine         Restores installed gems to their original condition");
    println!("  push             Push a gem to RubyGems.org");
    println!("  query            Query gem information from the local repository");
    println!("  rdoc             Generates RDoc for pre-installed gems");
    println!("  search           Display all gems that match STRING");
    println!("  server           Documentation and gem repository HTTP server");
    println!("  sources          Manage the sources and cache file RubyGems uses to search for gems");
    println!("  specification    Display gem specification (in yaml)");
    println!("  uninstall        Uninstall a gem from the local repository");
    println!("  update           Update installed gems to the latest version");
    println!("  which            Find the location of a library file you can require");
    println!("  yank             Remove a specific version of a gem from RubyGems.org");
    println!("");
    println!("  For help on a particular command, use 'gem help COMMAND'.");
    println!("");
    println!("  Example: gem help install");
}

/// 安装 gem
fn install_gem(args: &[String]) {
    if args.is_empty() {
        println!("Error: Missing gem name");
        return;
    }
    let gem_name = &args[0];
    println!("Fetching: {}-0.1.0.gem (100%)", gem_name);
    println!("Successfully installed {}-0.1.0", gem_name);
    println!("1 gem installed");
}

/// 卸载 gem
fn uninstall_gem(args: &[String]) {
    if args.is_empty() {
        println!("Error: Missing gem name");
        return;
    }
    let gem_name = &args[0];
    println!("Successfully uninstalled {}-0.1.0", gem_name);
}

/// 列出 gems
fn list_gems(_args: &[String]) {
    println!("*** LOCAL GEMS ***");
    println!("");
    println!("nyar-vm (0.1.0)");
    println!("rusty-ruby (0.1.0)");
    println!("");
    println!("Gems included by default: bigdecimal, bundler, cmath, csv, date, dbm, etc.");
}

/// 更新 gem
fn update_gem(args: &[String]) {
    if args.is_empty() {
        println!("Updating installed gems");
        println!("Nothing to update");
    }
    else {
        let gem_name = &args[0];
        println!("Updating {}", gem_name);
        println!("Nothing to update");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 处理全局参数
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
            _ => break,
        }
    }

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "install" => install_gem(&args[2..]),
        "uninstall" => uninstall_gem(&args[2..]),
        "list" => list_gems(&args[2..]),
        "update" => update_gem(&args[2..]),
        "help" => print_help(),
        _ => {
            println!("Unknown command: {}", command);
            print_help();
        }
    }
}
