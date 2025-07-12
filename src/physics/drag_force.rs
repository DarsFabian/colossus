use crate::physics::constants::{LAPSE_RATE, SEA_LEVEL_TEMPERATURE, SEA_LEVEL_DENSITY, AIR_MOLAR_MASS, UNIVERSAL_GAS_CONSTANT};

pub fn calc_troposphere_density(altitude: f64, gravity_acceleration: f64) -> f64 {
    let temp_ratio = 1.0 - (LAPSE_RATE * altitude) / SEA_LEVEL_TEMPERATURE;
    let exponent = (gravity_acceleration * AIR_MOLAR_MASS) / (LAPSE_RATE * UNIVERSAL_GAS_CONSTANT) - 1.0;
    let density_ratio = temp_ratio.powf(exponent);

    SEA_LEVEL_DENSITY * density_ratio
}

// Using 0.3 and 80 as Saturn V placeholder values
pub fn calc_drag_force(air_density: f64, speed: f64) -> f64 {
    0.5 * air_density * speed.powi(2) * 0.3 * 20.
}