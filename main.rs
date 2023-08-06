mod geo;
use geo::calculations::distance as distance_calc;
use rand::Rng;

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

fn main() {
    let route = [
        Waypoint::new("KCLE".to_string(), 41.4075, -81.851111),
        Waypoint::new("WEGEM".to_string(), 41.44560, -109.9900),
        Waypoint::new("KSLC".to_string(), 40.7861, -111.9822),
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<&Waypoint> = None;

    for current_waypoint in route.iter() {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(current_waypoint.clone());
                continue;
            }
            Some(previous_waypoint_value) => {
                let distance = distance_calc(
                    previous_waypoint_value.latitude,
                    previous_waypoint_value.longitude,
                    current_waypoint.latitude,
                    current_waypoint.longitude,
                );

                total_distance += distance;
                previous_waypoint = Option::from(current_waypoint.clone());

                println!(
                    "The distance between {} and {} is {:.1} kilometers",
                    previous_waypoint_value.name, current_waypoint.name, distance
                )
            }
        }
    }

    println!("Total distance is {:.2} kilometers", total_distance);

    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    println!("Random number is {}", random_number);
}
