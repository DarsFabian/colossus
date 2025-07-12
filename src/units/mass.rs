#[derive(Clone, Copy)]
pub struct Kg(f64);

#[derive(Clone, Copy)]
pub struct Gram(f64);

impl Kg {
    pub fn new(kg: f64) -> Kg {
        Kg(kg)
    }

    #[allow(dead_code)]
    pub fn from(grams: Gram) -> Kg {
        Kg(grams.as_f64() / 1000.0)
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }
}

impl Gram {
    #[allow(dead_code)]
    pub fn new(gramms: f64) -> Gram {
        Gram(gramms)
    }
    
    #[allow(dead_code)]
    pub fn from(kg: Kg) -> Gram {
        Gram(kg.as_f64() * 1000.0)
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }
}