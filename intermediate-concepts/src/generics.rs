
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

// You can enforce the same type
struct Rectangle<T> {
    height: T,
    width: T,
}

// Or different ones
struct Cube<T, U, V> {
    height: T,
    width: U,
    length: V,
}

fn struct_example() {
    
    let rect1 = Rectangle{height: 1, width: 2};
    let rect2 = Rectangle{height: 1.65, width: 2.22};

    let cube1 = Cube{height: 1, width: 2, length: 3};
    let cube2 = Cube{height: 1.85, width: -2, length: 3};
}



// In Functions

fn lookup_datatype<T>(object: T) {
    println!("{}", std::any::type_name::<T>());
}


fn functions_example() {
    
    lookup_datatype("jelly");
    lookup_datatype(1);
    lookup_datatype(5.84);
    lookup_datatype(Some(1));
}