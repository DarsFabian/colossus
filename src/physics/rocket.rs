#![allow(dead_code)]
use crate::{physics::{body::Body3, vectors::Vector3}, units::{mass::Kg, thrust::{N}}};


pub struct Rocket {
    position: Vector3,
    direction: Vector3,
    velocity: Vector3,
    mass: Kg,
    thrust: N
}

impl Rocket {
    pub fn new(position: Vector3, direction: Vector3, velocity: Vector3, mass: Kg, thrust: N) -> Rocket {
        Rocket { position, direction, velocity, mass, thrust }
    }

    pub fn get_pos(&self) -> Vector3 {
        self.position
    }

    pub fn get_velocity(&self) -> Vector3 {
        self.velocity
    }

    pub fn get_mass(&self) -> Kg {
        self.mass
    }

    pub fn get_thrust(&self) -> N {
        self.thrust
    }

    pub fn difference_vector(&self, other: &Body3) -> Vector3 {
        let vect_x: f64 = other.get_pos().get_x() - self.get_pos().get_x();
        let vect_y: f64 = other.get_pos().get_y() - self.get_pos().get_y();
        let vect_z: f64 = other.get_pos().get_z() - self.get_pos().get_z();
        Vector3::from(vect_x, vect_y, vect_z)
    }

    pub fn distance_from_obj(&self, other: &Body3) -> f64 {
        let vector: Vector3 = self.difference_vector(other);
        vector.magnitude() - other.get_radius()
    }

    pub fn update_position(&mut self) {
        self.position.add(&self.velocity);
    }

    pub fn apply_acceleration(&mut self, acceleration: &Vector3) {
        self.velocity.add(acceleration);
    }

    pub fn update(&mut self) {
        let thrust_accel: f64 = self.thrust.as_f64() / self.mass.as_f64();
        let mut thrust_vector: Vector3 = self.direction.clone();
        thrust_vector.mult(thrust_accel);
        self.apply_acceleration(&thrust_vector);

        todo!("Bodies gravitionnal pull.");
        
        self.update_position();
    }

}