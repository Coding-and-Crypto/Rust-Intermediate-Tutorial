
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
    let my_truck = Truck { color: String::from("green") };
    let my_car = Car { color: String::from("red") };
    println!("{}", my_car.forward());
    println!("{}", my_car.backward());
    println!("{}", my_truck.forward());
    println!("{}", my_truck.backward());
}
