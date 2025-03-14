use crate::functions::trigonometric::TrigonometricType;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum FuncType {
    LINEAR,
    QUADRATIC,
    TRIGONOMETRIC_SIN,
    TRIGONOMETRIC_COS,
}

#[derive(Clone, Copy, Deserialize)]
pub struct RandRange {
    pub min: i32,
    pub max: i32,
}

#[derive(Deserialize)]
pub struct GenOptions {
    pub function_type: FuncType,
    pub range: RandRange,
}

#[derive(Deserialize)]
pub struct CalcOptions {
    pub function_type: FuncType,
    pub range: RandRange,
    pub x: f32,
}

pub struct LinearF {
    pub k: f32,
    pub b: f32,
}

pub struct QuadraticF {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

pub struct TrigonometricF {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub ftype: TrigonometricType,
}

#[derive(Deserialize)]
pub struct Derivative {
    pub x: f32,
}
