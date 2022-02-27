
use std::process::Command;
use std::io::{Error, ErrorKind};

// Linux only
fn error_handling_example_1(dir: &str) {

    println!("\n\n");

    let mut list_cmd = Command::new("ls");

    match list_cmd.current_dir(dir).status() {
        Ok(cmd) => cmd,
        Err(e) => panic!("Error: {}", e),
    };

    println!("\n\n");
}

// Linux only
fn error_handling_example_2(dir: &str) {

    println!("\n\n");

    let mut list_cmd = Command::new("ls");

    let x = match list_cmd.current_dir(dir).status() {
        Ok(cmd) => Some(cmd),
        Err(e) => {
            println!("Directory not found!");
            None
        },
    };

    println!("\n\n");
}

// Linux only
fn error_handling_example_3(dir: &str) {

    println!("\n\n");

    let mut list_cmd = Command::new("ls");

    let x = match list_cmd.current_dir(dir).status() {
        Ok(cmd) => Some(cmd),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("Directory not found!");
                None
            },
            _ => panic!("An unexpected error has occurred!")
        },
    };

    println!("\n\n");
}

// Linux only
fn error_handling_example_4(dir: &str) -> Result<i32, Error> {

    println!("\n\n");

    let mut list_cmd = Command::new("ls");

    list_cmd.current_dir(dir).status()?;

    println!("\n\n");

    Ok(1)
}

fn main() {
    error_handling_example_1("src");
    error_handling_example_1("lib");
    error_handling_example_2("src");
    error_handling_example_2("lib");
    error_handling_example_3("src");
    error_handling_example_3("lib");
    error_handling_example_4("src");
    error_handling_example_4("lib");
}
