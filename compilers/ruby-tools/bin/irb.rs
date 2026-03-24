//! IRB (Interactive Ruby) tool
//! Provides interactive Ruby interpreter functionality

use clap::Parser;
use ruby::Ruby;
use ruby_tools::RustyRubyFrontend;
use ruby_types::RubyValue;
use std::io::{self, BufRead, Write};

/// IRB command-line tool
#[derive(Parser, Debug)]
#[command(name = "irb", version = "1.11.0", about = "Interactive Ruby shell")]
pub struct IrbArgs {
    /// Script file to execute
    script: Option<String>,

    /// Script arguments
    #[arg(last = true)]
    args: Vec<String>,

    /// Suppress read of ~/.irbrc
    #[arg(short = 'f')]
    suppress_irbrc: bool,

    /// Bc mode (load mathn, fraction or matrix)
    #[arg(short = 'm')]
    bc_mode: bool,

    /// Set $DEBUG to true (same as `ruby -d')
    #[arg(short = 'd')]
    debug: bool,

    /// Require the library before executing script
    #[arg(short = 'r', value_name = "library")]
    require_library: Option<String>,

    /// Specify $LOAD_PATH directory (may be used multiple times)
    #[arg(short = 'I', value_name = "path")]
    load_path: Option<String>,

    /// Set default encoding to UTF-8
    #[arg(short = 'U')]
    utf8: bool,
}

/// Start interactive session
fn start_interactive_session() {
    println!("irb 1.11.0 (2024-12-25) -- Interactive Ruby");
    println!("Type 'exit' or 'quit' to exit.");
    println!();

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let mut line_number = 1;

    // Create Ruby runtime
    match Ruby::new() {
        Ok(mut ruby) => {
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
                            match ruby.execute_script(trimmed_input) {
                                Ok(_) => {
                                    // Try to get the result from the last expression
                                    match ruby.get_global("$_").unwrap() {
                                        RubyValue::Nil => println!("=> nil"),
                                        value => println!("=> {:?}", value),
                                    }
                                }
                                Err(e) => {
                                    println!("Error: {:?}", e);
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
        }
        Err(e) => {
            println!("Error initializing Ruby runtime: {:?}", e);
        }
    }

    println!("Exiting irb...");
}

fn main() {
    let args = IrbArgs::parse();

    // Handle script file
    if let Some(_script) = args.script {
        println!("Error: irb does not support script files");
        return;
    }

    // Handle other parameters
    if args.suppress_irbrc {
        println!("Suppress ~/.irbrc option not yet implemented");
    }

    if args.bc_mode {
        println!("Bc mode not yet implemented");
    }

    if args.debug {
        println!("Debug mode not yet implemented");
    }

    if args.require_library.is_some() {
        println!("Require library option not yet implemented");
    }

    if args.load_path.is_some() {
        println!("Load path option not yet implemented");
    }

    if args.utf8 {
        println!("UTF-8 encoding option not yet implemented");
    }

    // Start interactive session
    start_interactive_session();
}
