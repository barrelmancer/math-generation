// y = a * sin(bx + c) + d
// y = a * cos(bx + c) + d

use rand::prelude::*;
use crate::utils::entities::{RandRange};

pub enum TrigType {
    SIN,
    COS,
}

pub struct Trigonometric {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub ftype: TrigType,
}

impl Trigonometric {
    pub fn new(r: RandRange, ftype: TrigType) -> Trigonometric {
        let mut rng = rand::rng();
        let e = rng.random_range((r.min as f32)..(r.max as f32));
        let a = if e == 0.0 { 1.0 } else { e };
        let b = rng.random_range((r.min as f32)..(r.max as f32));
        let c = rng.random_range((r.min as f32)..(r.max as f32));
        let d = rng.random_range((r.min as f32)..(r.max as f32));
        Trigonometric { a, b, c, d, ftype }
    }
    pub fn calculate(&self, x: i32) -> f32 {
        match self.ftype {
            TrigType::SIN => self.a * f32::sin(self.b * x as f32 + self.c) + self.d,
            TrigType::COS => self.a * f32::cos(self.b * x as f32 + self.c) + self.d,
        }
    }
    pub fn options(&self) -> String {
        format!("{{\"a\": {}, \"b\": {}, \"c\": {}, \"d\": {}}}", self.a, self.b, self.c, self.d)
    }
    pub fn show(&self) -> String {
        let mut result = String::from("y = ");
        if self.a != 1.0 && self.a != -1.0 {
            result += &self.a.to_string();
        }
        match self.ftype {
            TrigType::SIN => result += "sin(",
            TrigType::COS => result += "cos(",
        }
        if self.b != 1.0 && self.b != -1.0 {
            result += &self.b.to_string();
        }
        result += "x";
        if self.c != 0.0 {
            if self.c > 0.0 {
                result += " + ";
                result += &self.c.to_string();
            } else {
                result += " - ";
                result += &(-self.c).to_string();
            }
        }
        result += ") + ";
        result += &self.d.to_string();
        result
    }
}

