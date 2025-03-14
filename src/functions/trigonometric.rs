// y = a * sin(bx + c) + d
// y = a * cos(bx + c) + d

use crate::utils::entities::RandRange;
use rand::prelude::*;

pub enum TrigonometricType {
    SIN,
    COS,
}

pub struct Trigonometric {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub ftype: TrigonometricType,
}

impl Trigonometric {
    pub fn new(a: f32, b: f32, c: f32, d: f32, ftype: TrigonometricType) -> Self {
        Trigonometric { a, b, c, d, ftype }
    }
    pub fn new_rand(r: RandRange, ftype: TrigonometricType) -> Self {
        let mut rng = rand::rng();
        let e = rng.random_range((r.min as f32)..(r.max as f32));
        let a = if e == 0.0 { 1.0 } else { e };
        let b = rng.random_range((r.min as f32)..(r.max as f32));
        let c = rng.random_range((r.min as f32)..(r.max as f32));
        let d = rng.random_range((r.min as f32)..(r.max as f32));
        Trigonometric { a, b, c, d, ftype }
    }
}
