#![allow(dead_code)]

use rand::{thread_rng, Rng};

pub fn random_double() -> f64 {
    let mut rng = thread_rng();    
    rng.gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng(); 
    rng.gen_range(min..max)
}