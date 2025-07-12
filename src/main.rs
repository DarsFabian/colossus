mod units;
mod physics;

use physics::{vectors::Vector2, body::Body, constants::NEWTONIAN_CONSTANT_OF_GRAVITATION};
use units::mass::Kg;
use units::speed::{MetersPerSecond, KmPerHour};
use physics::constants::{EARTH_MASS_KG, EARTH_RADIUS};

use crate::physics::drag_force::{calc_drag_force, calc_troposphere_density};

fn main() {
    let earth_position: Vector2 = Vector2::from(40_000_000., 20_000_000.);
    let earth_velocity: Vector2 = Vector2::from(0.0, 0.0);
    let earth_mass: Kg = Kg::new(EARTH_MASS_KG);
    let earth_radius: f64 = EARTH_RADIUS;
    let earth: Body = Body::new(earth_position, earth_velocity, earth_mass, earth_radius);

    let rocket_position: Vector2 = Vector2::from(earth.get_pos().get_x(), earth.get_pos().get_y() + EARTH_RADIUS + 100.);
    let rocket_velocity: Vector2 = Vector2::from(0., 30.);
    let rocket_mass: Kg = Kg::new(1000.);
    let mut rocket: Body = Body::new(rocket_position, rocket_velocity, rocket_mass, 0.);

    for second in 0..1000 {

        let mut rocket_to_earth: Vector2 = rocket.difference_vector(&earth);

        let mass_product: f64 = earth.get_mass().as_f64() * rocket.get_mass().as_f64();
        let gravity_force: f64 = NEWTONIAN_CONSTANT_OF_GRAVITATION * (mass_product / rocket_to_earth.magnitude().powi(2));
        let gravity_acceleration_force: f64 = gravity_force / rocket.get_mass().as_f64();

        rocket_to_earth.normalize();
        rocket_to_earth.mult(gravity_acceleration_force);

        let rocket_altitude: f64 = rocket.distance_from_obj(&earth);
        let rocket_speed: f64 = rocket.get_velocity().magnitude();

        let air_density: f64 = calc_troposphere_density(rocket_altitude, gravity_acceleration_force);
        let drag_acceleration_force: f64 = calc_drag_force(air_density, rocket_speed) / rocket.get_mass().as_f64();

        let mut drag_vector: Vector2 = rocket.get_velocity();
        drag_vector.mult(-1.);
        drag_vector.normalize();
        drag_vector.mult(drag_acceleration_force);

        
        rocket.apply_acceleration(&rocket_to_earth);
        rocket.apply_acceleration(&drag_vector);

        let velocity_magnitude: f64 = rocket.get_velocity().magnitude();
        let speed_ms: MetersPerSecond = MetersPerSecond::new(velocity_magnitude);
        let speed_kmh: KmPerHour = KmPerHour::from_ms(&speed_ms);

        rocket.update_position();


        if rocket_altitude < 0.0 {
            break;
        }


        println!("Acceleration at second {:?}: {:?}m/s, {:?}km/h. Distance from earth's surface: {:?}km", second, speed_ms.as_f64(), speed_kmh.as_f64(), rocket_altitude / 1000.);
        println!("Drag acceleration: {}", drag_acceleration_force);
    }
}
