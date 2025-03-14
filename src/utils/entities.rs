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
    pub x: i32,
}

