use super::vectors::Vector2;
use crate::units::mass::Kg;

#[derive(Clone, Copy)]
pub struct Body {
    position: Vector2,
    velocity: Vector2,
    mass: Kg,
    _radius: f64,
}

impl Body {
    pub fn new(position: Vector2, velocity: Vector2, mass: Kg, _radius: f64) -> Body {
        Body { position, velocity, mass, _radius }
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

    pub fn difference_vector(&self, other: &Body) -> Vector2 {
        let vect_x: f64 = other.get_pos().get_x() - self.get_pos().get_x();
        let vect_y: f64 = other.get_pos().get_y() - self.get_pos().get_y();
        Vector2::from(vect_x, vect_y)
    }

    pub fn distance_from_obj(&self, other: &Body) -> f64 {
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