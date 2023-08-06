pub mod calculations {
    const EARTH_RADIUS_IN_KM: f64 = 6371.0;
    
    pub fn distance(
        start_latitude: f64,
        start_longitude: f64,
        end_latitude: f64,
        end_longitude: f64,
    ) -> f64 {
        let previous_waypoint_radians = start_latitude.to_radians();
        let waypoint_radians = end_latitude.to_radians();
    
        let delta_latitude = (start_latitude - end_latitude).to_radians();
        let delta_longitude = (start_longitude - end_longitude).to_radians();
    
        let inner_city_angle = f64::powi((delta_latitude / 2.0).sin(), 2)
            + previous_waypoint_radians.cos()
                * waypoint_radians.cos()
                * f64::powi((delta_longitude / 2.0).sin(), 2);
    
        let central_angle = 2.0 * inner_city_angle.sqrt().asin();
        let distance = central_angle * EARTH_RADIUS_IN_KM;
    
        return distance;
    }
}
