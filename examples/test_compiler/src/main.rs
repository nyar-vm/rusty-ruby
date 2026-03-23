use ruby::Ruby;
use std::{env, path::Path};

fn main() {
    let mut ruby = Ruby::new().unwrap();

    // Get the project root directory
    let current_dir = env::current_dir().unwrap();
    let project_root = if current_dir.ends_with("compilers/ruby") { current_dir.parent().unwrap().parent().unwrap() } else { &current_dir };

    // Read the test script
    let test_file = project_root.join("test.rb");
    println!("Reading test file: {:?}", test_file);
    let script = std::fs::read_to_string(test_file).unwrap();

    // Execute the script
    match ruby.execute_script(&script) {
        Ok(_) => println!("Script executed successfully"),
        Err(e) => println!("Error executing script: {:?}", e),
    }
}
