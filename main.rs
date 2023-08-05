fn main() {
    const EARTH_RADIUS_IN_KM: f64 = 6371.0;

    let route = [
        ("KCLE", 41.4075, -81.851111),
        ("WEGEM", 41.44560, -109.9900),
        ("KSLC", 40.7861, -111.9822),
    ];

    let mut total_distance = 0.0;
    let mut previous_waypoint: Option<(&str, f64, f64)> = None;

    for current_waypoint in route.iter() {
        match previous_waypoint {
            None => {
                previous_waypoint = Option::from(current_waypoint.clone());
                continue;
            }
            Some(previous_waypoint_value) => {
                let previous_waypoint_radians = previous_waypoint_value.1.to_radians();
                let waypoint_radians = current_waypoint.1.to_radians();

                let delta_latitude = (previous_waypoint_value.1 - current_waypoint.1).to_radians();
                let delta_longitude = (previous_waypoint_value.2 - current_waypoint.2).to_radians();

                let inner_city_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
                    + previous_waypoint_radians.cos()
                        * waypoint_radians.cos()
                        * f64::powi((delta_longitude / 2.0).sin(), 2);

                let central_angle = 2.0 * inner_city_angle.sqrt().asin();
                let distance = central_angle * EARTH_RADIUS_IN_KM;

                total_distance += distance;
                previous_waypoint = Option::from(current_waypoint.clone());

                println!(
                    "The distance between {} and {} is {:.1} kilometers",
                    previous_waypoint_value.0, current_waypoint.0, distance
                )
            }
        }
    }

    println!("Total distance is {:.2} kilometers", total_distance)
}
