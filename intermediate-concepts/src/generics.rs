
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

enum Custom<T, U> {
    EXAMPLE(T),
    SAMPLE(U),
}


// Structs

struct Rectangle<T> {
    height: T,
    width: T,
}

struct Cube<T, U, V> {
    height: T,
    width: U,
    length: V,
}

fn struct_example() {

    let rect1 = Rectangle{height: 1, width: 2};
    let rect1 = Rectangle{height: 1.65, width: 2.22};

    let cube1 = Cube{height: 1, width: 2, length: 3};
    let cube1 = Cube{height: 1.54, width: 2, length: 3.75};
}


// Functions

fn sum_of_numbers<T: std::ops::Mul<Output = T>>(num1: T, num2: T) -> T {
    num1 * num2
}

fn lookup_datatype<T>(object: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    println!("{}", sum_of_numbers(1, 2));
    lookup_datatype(1);
    lookup_datatype(1.96);
    lookup_datatype("string_slice");
}