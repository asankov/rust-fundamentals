use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug)]
struct NavAid<T, U> {
    name: String,
    frequency: T,
    data: U,
}

fn main() {
    let vor = NavAid {
        name: String::from("DQN"),
        frequency: 114.5,
        data: String::from("DQN is a VOR"),
    };
    let ndb_data: Option<String> = Option::None;
    let ndb = NavAid {
        name: String::from("HKF"),
        frequency: 239,
        data: ndb_data,
    };
    println!("VOR information is {:?}", vor);
    println!("NDB information is {:?}", ndb);

    println!("{}", add(1, 2));
    println!("{}", add(1.1, 2.2));
}

fn add<T>(operand1: T, operand2: T) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    return operand1 + operand2;
}
