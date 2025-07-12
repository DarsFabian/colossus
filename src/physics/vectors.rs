#[derive(Clone, Copy, Debug)]
pub struct Vector2 {
    x: f64,
    y: f64
}

impl Vector2 {
    pub fn from(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn mult(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }

    pub fn add(&mut self, other: &Vector2) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    #[allow(dead_code)]
    pub fn norm(&self) -> Vector2 {
        let mag: f64 = self.magnitude();
        let x: f64 = self.x / mag;
        let y: f64 = self.y / mag;
        Vector2 { x, y }
    }

    pub fn normalize(&mut self) {
        let mag: f64 = self.magnitude();
        self.x /= mag;
        self.y /= mag;
    }
}