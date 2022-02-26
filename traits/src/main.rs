
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
    let my_truck = Truck { color: String::from("green") };
    let my_car = Car { color: String::from("red") };
    println!("{}", my_car.forward());
    println!("{}", my_car.backward());
    println!("{}", my_truck.forward());
    println!("{}", my_truck.backward());
}
