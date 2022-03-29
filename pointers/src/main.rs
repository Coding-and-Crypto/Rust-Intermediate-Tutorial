

// Pointers

// Lifetimes

fn lifetimes() {
    let x = 5;

    println!("{}", &x);

    println!("{}", &x);
}

fn specific_lifetimes<'a>(val1: &str, val2: &str) -> &'a str {

    println!("{}, {}", val1, val2);

    let q = "some string slice q";
    q
}

const constant: &'static str = "some constant static string slice.";

// Strings vs String Slices (&str)

fn burn_this_string(string: &String) {
    if string.len() < 0 {
        panic![];
    }
}

fn burn_this_string_slice(string_slice: &str) {
    if string_slice.len() < 0 {
        panic![];
    }
}

fn main() {

    let some_string = String::from("Johnny");
    burn_this_string(&some_string);
    println!("{}", some_string);

    let some_string_slice = "Johnny";
    burn_this_string_slice(some_string_slice);
    println!("{}", some_string_slice);

}
