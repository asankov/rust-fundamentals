use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut flights: Vec<&str> = Vec::new();
    let vec_macro = vec![1, 2, 3, 4];

    // push adds a new value
    flights.push("DA113\t to Boston depars at 18:30");
    flights.push("DA98\t to London departs at 9:30");

    for flight in flights.iter() {
        println!("{}", flight);
    }

    // might panic if the collection does not have 2 elements
    let second = flights[1];
    println!("The second flight is {}", second);

    // save way to get the element at given index
    let fifth = flights.get(4);
    match fifth {
        Some(flight) => println!("The fifth flight is {}", flight),
        None => println!("There is no flight at this index."),
    }

    if let Some(flight) = flights.get(4) {
        println!("The fifth flight is {}", flight)
    }

    // insert element at given index
    flights.insert(2, "DA918\t to Orlando leaves at 9:48");
    for flight in flights.iter() {
        println!("{}", flight);
    }

    // remove element at given index
    flights.remove(1);
    for flight in flights.iter() {
        println!("{}", flight);
    }

    let mut flights: VecDeque<&str> = VecDeque::new();

    flights.push_front("DA113\t to Boston depars at 18:30");
    flights.push_back("DA98\t to London departs at 9:30");
    flights.push_front("DA981\t to New York departs at 9:31");

    println!("There are {} number of flights", flights.len());
    for flight in flights.iter() {
        println!("{}", flight);
    }
    let contains_to_new_york = flights.contains(&"DA981\t to New York departs at 9:31");
    println!("{}", contains_to_new_york);

    flights.clear();
    println!("There are {} number of flights", flights.len());
    for flight in flights.iter() {
        println!("{}", flight);
    }

    let mut flights = HashMap::new();

    let flight_number = "DA918";
    flights.insert(flight_number, ("11:12", "Orlando"));
    flights.insert("DA428", ("12:05", "Salt Lake City"));

    let flight = flights.get(flight_number);
    // might panic, not recommended
    let (time, destination) = flight.unwrap();
    println!("{} {}", time, destination);

    if !flights.contains_key(flight_number) {
        flights.insert(flight_number, ("12:00", "Puerto Rico"));
    } else {
        println!("Flight {} already exists", flight_number)
    }

    let flight = flights.get(flight_number);
    // might panic, not recommended
    let (time, destination) = flight.unwrap();
    println!("{} {}", time, destination);

    let mut flights = HashSet::new();

    flights.insert(("DA918", "11:12", "Orlando"));
    flights.insert(("DA428", "12:05", "Salt Lake City"));

    for flight in flights.iter() {
        println!("{:?}", flight);
    }
}
