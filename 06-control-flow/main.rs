enum NumerationAids {
    NDB(u16),
    VOR(String, f32),
    VORDME(String, f32),
    FIX {
        name: String,
        longitude: f64,
        latitude: f64,
    },
}
fn print_nav_aid(navaid: &NumerationAids) {
    match navaid {
        NumerationAids::NDB(khz) => {
            println!("NDB frequency is {} khz", khz)
        }
        NumerationAids::VOR(id, khz) => {
            println!("VOR identifier is {} and its frequency is {}", id, khz)
        }
        NumerationAids::VORDME(id, khz) => {
            println!("VORDME identifier is {} and its frequency is {}", id, khz)
        }
        NumerationAids::FIX {
            name,
            longitude,
            latitude,
        } => {
            println!(
                "FIX {} is at {} latitude and {} longitude",
                name, latitude, longitude
            )
        }
    }
}

fn main() {
    let phrase = String::from("Duck Airlines");
    let letter = phrase.chars().nth(15);

    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("no value"),
    };

    println!("{}", value);

    let ndb = NumerationAids::NDB(385);
    let vor = NumerationAids::VOR(String::from("DQN"), 114.5);
    let vor_dme = NumerationAids::VORDME(String::from("SGH"), 113.2);
    let fix = NumerationAids::FIX {
        name: String::from("TARRY"),
        longitude: 40.0533,
        latitude: -83.1993,
    };

    print_nav_aid(&ndb);
    print_nav_aid(&vor);
    print_nav_aid(&vor_dme);
    print_nav_aid(&fix);

    let animal = get();

    if let animal = "Duck" {
        println!("Quack {}", animal)
    }
    if let animal = "Dog" {
        println!("Bark {}", animal)
    }

    let aircrafts = ["Boeing", "Airbus"];

    for aircraft in aircrafts.iter() {
        println!("{}", aircraft)
    }
}

fn get() -> String {
    return String::from("Duck");
}
