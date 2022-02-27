
pub trait Vehicle {
    fn forward() -> String;
    fn backward() -> String;
}

pub struct Truck {
    color: String,
}

pub struct Car {
    color: String,
}

impl Vehicle for Truck {
    fn forward() -> String {
        String::from("Driving my truck forward!")
    }
    fn backward() -> String {
        String::from("Backing my truck up...")
    }
}

impl Vehicle for Car {
    fn forward() -> String {
        String::from("Driving my car forward!")
    }
    fn backward() -> String {
        String::from("Backing my car up...")
    }
}

fn main() {
    println!("{}", Car::forward());
    println!("{}", Car::backward());
    println!("{}", Truck::forward());
    println!("{}", Truck::backward());
}
