// y = ax^2 + bx + c

use crate::utils::entities::RandRange;
use rand::prelude::*;

pub struct Quadratic {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Quadratic {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Quadratic { a, b, c }
    }
    pub fn new_rand(r: RandRange) -> Self {
        let mut rng = rand::rng();
        let e = rng.random_range((r.min as f32)..(r.max as f32));
        let a = if e == 0.0 { 1.0 } else { e };
        let b = rng.random_range((r.min as f32)..(r.max as f32));
        let c = rng.random_range((r.min as f32)..(r.max as f32));
        Quadratic { a, b, c }
    }
}
