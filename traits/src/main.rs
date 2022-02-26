
pub trait Vehicle {
    fn forward(&self) -> String;
    fn backward(&self) -> String;
}

pub struct Truck {
    color: String,
}

pub struct Car {
    color: String,
}

impl Vehicle for Truck {
    fn forward(&self) -> String {
        String::from(format!("Driving my {} truck forward!", self.color))
    }
    fn backward(&self) -> String {
        String::from(format!("Backing my {} truck up...", self.color))
    }
}

impl Vehicle for Car {
    fn forward(&self) -> String {
        String::from(format!("Driving my {} car forward!", self.color))
    }
    fn backward(&self) -> String {
        String::from(format!("Backing my {} car up...", self.color))
    }
}


fn main() {
    println!("Hello, world!");
}
