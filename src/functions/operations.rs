use crate::functions::{
    linear::Linear,
    quadratic::Quadratic,
    trigonometric::{Trigonometric, TrigonometricType},
};

pub trait Operations {
    fn calculate(&self, x: f32) -> f32;
    fn show(&self) -> String;
    fn options(&self) -> String;
}

impl Operations for Linear {
    fn calculate(&self, x: f32) -> f32 {
        self.k * x + self.b
    }
    fn options(&self) -> String {
        format!("{{\"k\": {}, \"b\": {}}}", self.k, self.b)
    }
    fn show(&self) -> String {
        let mut result = String::from("y = ");
        if self.k != 1.0 && self.k != -1.0 {
            result += &self.k.to_string();
        }
        result += "x";
        if self.b != 0.0 {
            if self.b > 0.0 {
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
impl Operations for Quadratic {
    fn calculate(&self, x: f32) -> f32 {
        self.a * x * x + self.b * x + self.c
    }
    fn options(&self) -> String {
        format!(
            "{{\"a\": {}, \"b\": {}, \"c\": {}}}",
            self.a, self.b, self.c
        )
    }
    fn show(&self) -> String {
        let mut result = String::from("y = ");
        if self.a != 1.0 && self.a != -1.0 {
            result += &self.a.to_string();
        }
        result += "x^2";
        if self.b != 0.0 {
            if self.b > 0.0 {
                result += " + ";
            } else {
                result += " - ";
            }
            if self.b > 1.0 {
                result += &self.b.to_string();
            } else if self.b < -1.0 {
                result += &(-self.b).to_string();
            }
            result += "x";
        }
        if self.c != 0.0 {
            if self.c > 0.0 {
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
impl Operations for Trigonometric {
    fn calculate(&self, x: f32) -> f32 {
        match self.ftype {
            TrigonometricType::SIN => self.a * f32::sin(self.b * x + self.c) + self.d,
            TrigonometricType::COS => self.a * f32::cos(self.b * x + self.c) + self.d,
        }
    }
    fn options(&self) -> String {
        format!(
            "{{\"a\": {}, \"b\": {}, \"c\": {}, \"d\": {}}}",
            self.a, self.b, self.c, self.d
        )
    }
    fn show(&self) -> String {
        let mut result = String::from("y = ");
        if self.a != 1.0 && self.a != -1.0 {
            result += &self.a.to_string();
        }
        match self.ftype {
            TrigonometricType::SIN => result += "sin(",
            TrigonometricType::COS => result += "cos(",
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
