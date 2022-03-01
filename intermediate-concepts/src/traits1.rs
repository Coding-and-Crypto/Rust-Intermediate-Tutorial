
pub trait Vehicle {

    fn forward() -> String;
    fn backward() -> String;

    fn turn_ignition() -> String {
        String::from("vroom vroom")
    }
}

pub struct Car {
    color: String,
}

impl Vehicle for Car {
    fn forward() -> String {
        String::from("Driving my car forward")
    }
    fn backward() -> String {
        String::from("Backing my car up...")
    }
}

pub struct Truck {
    color: String,
}

impl Vehicle for Truck {
    fn forward() -> String {
        String::from("Driving my truck forward")
    }
    fn backward() -> String {
        String::from("Backing my truck up...")
    }
}

fn main() {
    println!("{}", Car::turn_ignition());
    println!("{}", Car::forward());
    println!("{}", Car::backward());
    println!("{}", Truck::turn_ignition());
    println!("{}", Truck::forward());
    println!("{}", Truck::backward());
}