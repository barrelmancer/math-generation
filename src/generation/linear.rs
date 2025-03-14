// y = kx + b

use rand::prelude::*;
use crate::utils::entities::{RandRange};

pub struct Linear {
    pub k: i32,
    pub b: i32,
}
impl Linear {
    pub fn new(r: RandRange) -> Linear {
        let mut rng = rand::rng();
        let e = rng.random_range(r.min..r.max);
        let k = if e == 0 { 1 } else { e };
        let b = rng.random_range(r.min..r.max);
        Linear { k, b }
    }
    pub fn calculate(&self, x: i32) -> i32 {
        self.k * x + self.b
    }
    pub fn options(&self) -> String {
        format!("{{\"k\": {}, \"b\": {}}}", self.k, self.b)
    }
    pub fn show(&self) -> String {
        let mut result = String::from("y = ");
        if self.k != 1 && self.k != -1 {
            result += &self.k.to_string();
        }
        result += "x";
        if self.b != 0 {
            if self.b > 0 {
                result += " + ";
                result += &self.b.to_string();
            } else {
                result += " - ";
                result += &(-self.b).to_string();
            }
        }
        result
    }
}

