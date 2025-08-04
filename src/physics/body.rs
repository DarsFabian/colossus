#![allow(dead_code)]

use super::vectors::Vector2;
use crate::{physics::vectors::Vector3, units::mass::Kg};

#[derive(Clone, Copy)]
pub struct Body2 {
    position: Vector2,
    velocity: Vector2,
    mass: Kg,
    _radius: f64,
}

impl Body2 {
    pub fn new(position: Vector2, velocity: Vector2, mass: Kg, _radius: f64) -> Body2 {
        Body2 { position, velocity, mass, _radius }
    }

    pub fn get_pos(&self) -> Vector2 {
        self.position
    }

    pub fn get_velocity(&self) -> Vector2 {
        self.velocity
    }

    pub fn get_mass(&self) -> Kg {
        self.mass
    }

    #[allow(dead_code)]
    pub fn get_radius(&self) -> f64 {
        self._radius
    }

    pub fn difference_vector(&self, other: &Body2) -> Vector2 {
        let vect_x: f64 = other.get_pos().get_x() - self.get_pos().get_x();
        let vect_y: f64 = other.get_pos().get_y() - self.get_pos().get_y();
        Vector2::from(vect_x, vect_y)
    }

    pub fn distance_from_obj(&self, other: &Body2) -> f64 {
        let vector: Vector2 = self.difference_vector(other);
        vector.magnitude() - self._radius - other._radius
    }

    pub fn update_position(&mut self) {
        self.position.add(&self.velocity);
    }

    pub fn apply_acceleration(&mut self, acceleration: &Vector2) {
        self.velocity.add(acceleration);
    }
}

pub struct Body3 {
    position: Vector3,
    velocity: Vector3,
    mass: Kg,
    _radius: f64
}

impl Body3 {
    pub fn new(position: Vector3, velocity: Vector3, mass: Kg, _radius: f64) -> Body3 {
        Body3 { position, velocity, mass, _radius }
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

    pub fn get_radius(&self) -> f64 {
        self._radius
    }

    pub fn difference_vector(&self, other: &Body3) -> Vector3 {
        let vect_x: f64 = other.get_pos().get_x() - self.get_pos().get_x();
        let vect_y: f64 = other.get_pos().get_y() - self.get_pos().get_y();
        let vect_z: f64 = other.get_pos().get_z() - self.get_pos().get_z();
        Vector3::from(vect_x, vect_y, vect_z)
    }

    pub fn distance_from_obj(&self, other: &Body3) -> f64 {
        let vector: Vector3 = self.difference_vector(other);
        vector.magnitude() - self._radius - other._radius
    }

    pub fn update_position(&mut self) {
        self.position.add(&self.velocity);
    }

    pub fn apply_acceleration(&mut self, acceleration: &Vector3) {
        self.velocity.add(acceleration);
    }
}