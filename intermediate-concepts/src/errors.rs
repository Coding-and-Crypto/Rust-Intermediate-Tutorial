
use std::process::Command;

// Linux only
fn error_handling_example(dir: &str) {

    println!("\n\n");

    let mut list_cmd = Command::new("ls");

    match list_cmd.current_dir(dir).status() {
        Ok(cmd) => cmd,
        Err(e) => panic!("Error: {}", e),
    };

    println!("\n\n");
}

fn main() {
    error_handling_example("src");
    error_handling_example("lib");
}
