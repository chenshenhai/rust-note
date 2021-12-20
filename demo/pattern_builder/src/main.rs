use std::fmt::Debug;

#[derive(Debug)]
pub struct Car {
    _name: String,
    _time: String,
}

pub struct CarBuilder {
    time: String,
}

impl CarBuilder {
    pub fn new() -> Self {
        Self {
            time: String::from("0000-00-00 00:00:00")
        }
    }

    pub fn set_time(mut self, time: String) -> CarBuilder {
        self.time = time;
        self
    }

    pub fn build(&self, name: String) -> Car {
        let car = Car {
            _time: self.time.to_string(),
            _name: name,
        };
        car
    }
}

fn main() {
    println!("Hello, world!");
    let car = CarBuilder::new()
        .set_time(String::from("2021-01-01: 00:00:01"))
        .build(String::from("HelloCar"));
    println!("car = {:?}", car);
}
