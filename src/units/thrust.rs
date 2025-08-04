#![allow(dead_code)]
#[derive(Clone, Copy)]
pub struct N(f64);

impl N {
    pub fn new(n: f64) -> N {
        N(n)
    }

    pub fn as_f64(&self) -> f64 {
        self.0
    }
}