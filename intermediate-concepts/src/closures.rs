

fn closures_example_1(number1: i32, number2: i32) -> i32 {
    // Closure
    let new_number = |x| {
        println!("Multiplying {} by 2", x);
        x * 2
    };
    if number1 > number2 {
        new_number(number1)
    } else {
        new_number(number2)
    }
}

fn closures_example_2() {
    let some_printout = |x| {
        println!("Print out: {}", x);
        x
    };
    println!("{}", some_printout(1));
    println!("{}", some_printout("hello"));
}


fn main() {
    let x = closures_example_1(1, 3);
    println!("{}", x);
    let y = closures_example_1(6, 3);
    println!("{}", y);
}