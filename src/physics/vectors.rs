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

#[derive(Clone, Copy)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3 {

    #[allow(dead_code)]
    pub fn from(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    #[allow(dead_code)]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    #[allow(dead_code)]
    pub fn get_y(&self) -> f64 {
        self.y
    }

    #[allow(dead_code)]
    pub fn get_z(&self) -> f64 {
        self.z
    }

    #[allow(dead_code)]
    pub fn mult(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }

    #[allow(dead_code)]
    pub fn add(&mut self, other: &Vector3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    #[allow(dead_code)]
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    #[allow(dead_code)]
    pub fn norm(&self) -> Vector3 {
        let mag: f64 = self.magnitude();
        let x: f64 = self.x / mag;
        let y: f64 = self.y / mag;
        let z: f64 = self.z / mag;
        Vector3 { x, y, z }
    }

    #[allow(dead_code)]
    pub fn normalize(&mut self) {
        let mag: f64 = self.magnitude();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
    }
}