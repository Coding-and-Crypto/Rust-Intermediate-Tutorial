
// Generics

// enum Option<T> {
//     Some(T),
//     None,
// }

fn option_example(x: i32) -> Option<String> {
    match x > 2 {
        true => Some(String::from("result")),
        false => None,
    }
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn error_handling_example_1(dir: &str) {

    println!("\n\n");

    let mut list_cmd = std::process::Command::new("ls");

    match list_cmd.current_dir(dir).status() {
        Ok(cmd) => cmd,
        Err(e) => panic!("Error: {}", e),
    };

    println!("\n\n");
}



// In Structs



// In Functions