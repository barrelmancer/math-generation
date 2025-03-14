// y = kx + b
use crate::utils::entities::RandRange;
use rand::prelude::*;

pub struct Linear {
    pub k: f32,
    pub b: f32,
}

impl Linear {
    pub fn new(k: f32, b: f32) -> Self {
        Linear { k, b }
    }
    pub fn new_rand(r: RandRange) -> Self {
        let mut rng = rand::rng();
        let e = rng.random_range((r.min as f32)..(r.max as f32));
        let k = if e == 0.0 { 1.0 } else { e };
        let b = rng.random_range((r.min as f32)..(r.max as f32));
        Linear { k, b }
    }
}
