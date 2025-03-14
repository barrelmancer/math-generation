// y = ax^2 + bx + c

use rand::prelude::*;
use crate::utils::entities::{RandRange};

pub struct Quadratic {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

impl Quadratic {
    pub fn new(r: RandRange) -> Quadratic {
        let mut rng = rand::rng();
        let e = rng.random_range(r.min..r.max);
        let a = if e == 0 { 1 } else { e };
        let b = rng.random_range(r.min..r.max);
        let c = rng.random_range(r.min..r.max);
        Quadratic { a, b, c }
    }
    pub fn calculate(&self, x: i32) -> i32 {
        self.a * x * x + self.b * x + self.c
    }
    pub fn options(&self) -> String {
        format!("{{\"a\": {}, \"b\": {}, \"c\": {}}}", self.a, self.b, self.c)
    }
    pub fn show(&self) -> String {
        let mut result = String::from("y = ");
        if self.a != 1 && self.a != -1 {
            result += &self.a.to_string();
        }
        result += "x^2";
        if self.b != 0 {
            if self.b > 0 {
                result += " + ";
            } else {
                result += " - ";
            }
            if self.b > 1{
                result += &self.b.to_string();
            } else if self.b < -1 {
                result += &(-self.b).to_string();
            }
            result += "x";
        }
        if self.c != 0 {
            if self.c > 0 {
                result += " + ";
                result += &self.c.to_string();
            } else {
                result += " - ";
                result += &(-self.c).to_string();
            }
        }
        result
    }
}

