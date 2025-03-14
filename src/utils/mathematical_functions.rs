use crate::functions::operations::Operations;

pub fn derivative(l: &dyn Operations, x: f32) -> f32 {
    (l.calculate(x + 1e-5) - l.calculate(x)) / 1e-5
}
