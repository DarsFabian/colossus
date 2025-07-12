pub struct MetersPerSecond(f64);
pub struct KmPerHour(f64);

impl MetersPerSecond {
    #[allow(dead_code)]
    pub fn from_kmh(speed: &KmPerHour) -> MetersPerSecond {
        MetersPerSecond(speed.as_f64() / 3.6)
    }
    
    pub fn new(speed: f64) -> MetersPerSecond {
        MetersPerSecond(speed)
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl KmPerHour {
    pub fn from_ms(speed: &MetersPerSecond) -> KmPerHour {
        KmPerHour(speed.as_f64() * 3.6)
    }

    #[allow(dead_code)]
    pub fn new(speed: f64) -> KmPerHour {
        KmPerHour(speed)
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }
}