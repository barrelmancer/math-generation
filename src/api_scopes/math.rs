use crate::utils::entities::Derivative;
use crate::utils::mathematical_functions::derivative;

use crate::functions::{
    linear::Linear,
    quadratic::Quadratic,
    trigonometric::{Trigonometric, TrigonometricType},
};

use actix_web::{HttpResponse, web};

pub async fn scope_derivative(data: web::Json<Derivative>) -> HttpResponse {
    let s = Quadratic::new(1.0, 1.0, 1.0);
    let d = derivative(&s, data.x);
    HttpResponse::Ok().json(d)
}
