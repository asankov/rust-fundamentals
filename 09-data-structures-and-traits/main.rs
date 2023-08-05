struct Waypoint {
    name: String,
    latitude: f64,
    longitude: f64,
}

impl Waypoint {
    fn new(name: String, latitude: f64, longitude: f64) -> Self {
        Waypoint {
            name,
            latitude,
            longitude,
        }
    }
}

struct Segment {
    start: Waypoint,
    end: Waypoint,
}

impl Segment {
    fn new(start: Waypoint, end: Waypoint) -> Self {
        Segment { start, end }
    }

    fn distance(&self) -> f32 {
        const EARTH_RADIUS_IN_KM: f64 = 6371.0;

        let start_radians = self.start.latitude.to_radians();
        let end_radians = self.end.latitude.to_radians();

        let delta_latitude = (self.start.latitude - self.end.latitude).to_radians();
        let delta_longitude = (self.start.longitude - self.end.longitude).to_radians();

        let inner_city_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
            + start_radians.cos() * end_radians.cos() * f64::powi((delta_longitude / 2.0).sin(), 2);

        let central_angle = 2.0 * inner_city_angle.sqrt().asin();
        let distance = EARTH_RADIUS_IN_KM as f32 * central_angle as f32;

        return distance as f32;
    }
}

struct Boeing {
    required_crew: u8,
    range: u16,
}

struct Airbus {
    required_crew: u8,
    range: u16,
}

trait Flight {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool;
}

impl Flight for Boeing {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 280 > distance) {
            return true;
        }
        return false;
    }
}
impl Flight for Airbus {
    fn is_legal(&self, required_crew: u8, available_crew: u8, range: u16, distance: u16) -> bool {
        if (available_crew >= required_crew) && (range + 150 > distance) {
            return true;
        }
        return false;
    }
}

fn main() {
    let mut kcle = Waypoint {
        name: "KCLE".to_string(),
        latitude: 41.4075,
        longitude: -81.851111,
    };

    let mut kslc = Waypoint {
        name: "KSLC".to_string(),
        latitude: 40.7861,
        longitude: -111.9822,
    };

    let kcle_kslc = Segment::new(kcle, kslc);
    let distance = kcle_kslc.distance();

    println!("Distance is {:.2} kilometers", distance);

    let boeing = Boeing {
        required_crew: 5,
        range: 3000,
    };
    let airbus = Airbus {
        required_crew: 6,
        range: 2500,
    };

    let boeing_is_legal = boeing.is_legal(boeing.required_crew, 6, boeing.range, 3000);
    let airbus_is_legal = airbus.is_legal(airbus.required_crew, 6, airbus.range, 3000);

    println!(
        "Boeing is legal? {}\nAirbus is legal? {}",
        boeing_is_legal, airbus_is_legal
    );
}
