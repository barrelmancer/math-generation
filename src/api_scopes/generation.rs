use crate::functions::{
    linear::Linear,
    operations::Operations,
    quadratic::Quadratic,
    trigonometric::{Trigonometric, TrigonometricType},
};
use crate::utils::entities::{CalcOptions, FuncType, GenOptions};
use actix_web::{HttpResponse, web};

pub async fn options(data: web::Json<GenOptions>) -> HttpResponse {
    let r = data.range;
    match data.function_type {
        FuncType::LINEAR => {
            let l = Linear::new_rand(r);
            HttpResponse::Ok().json(l.options())
        }
        FuncType::QUADRATIC => {
            let q = Quadratic::new_rand(r);
            HttpResponse::Ok().json(q.options())
        }
        FuncType::TRIGONOMETRIC_SIN => {
            let t = Trigonometric::new_rand(r, TrigonometricType::SIN);
            HttpResponse::Ok().json(t.options())
        }
        FuncType::TRIGONOMETRIC_COS => {
            let t = Trigonometric::new_rand(r, TrigonometricType::COS);
            HttpResponse::Ok().json(t.options())
        }
    }
}
pub async fn get_all_information(data: web::Json<CalcOptions>) -> HttpResponse {
    let r = data.range;
    match data.function_type {
        FuncType::LINEAR => {
            let l = Linear::new_rand(r);
            HttpResponse::Ok().json((l.options(), l.show(), l.calculate(data.x)))
        }
        FuncType::QUADRATIC => {
            let q = Quadratic::new_rand(r);
            HttpResponse::Ok().json((q.options(), q.show(), q.calculate(data.x)))
        }
        FuncType::TRIGONOMETRIC_SIN => {
            let t = Trigonometric::new_rand(r, TrigonometricType::SIN);
            HttpResponse::Ok().json((t.options(), t.show(), t.calculate(data.x)))
        }
        FuncType::TRIGONOMETRIC_COS => {
            let t = Trigonometric::new_rand(r, TrigonometricType::COS);
            HttpResponse::Ok().json((t.options(), t.show(), t.calculate(data.x)))
        }
    }
}
pub async fn equation(data: web::Json<GenOptions>) -> HttpResponse {
    let r = data.range;
    match data.function_type {
        FuncType::LINEAR => {
            let l = Linear::new_rand(r);
            HttpResponse::Ok().json(l.show())
        }
        FuncType::QUADRATIC => {
            let q = Quadratic::new_rand(r);
            HttpResponse::Ok().json(q.show())
        }
        FuncType::TRIGONOMETRIC_SIN => {
            let t = Trigonometric::new_rand(r, TrigonometricType::SIN);
            HttpResponse::Ok().json(t.show())
        }
        FuncType::TRIGONOMETRIC_COS => {
            let t = Trigonometric::new_rand(r, TrigonometricType::COS);
            HttpResponse::Ok().json(t.show())
        }
    }
}
pub async fn calculate(data: web::Json<CalcOptions>) -> HttpResponse {
    let r = data.range;
    match data.function_type {
        FuncType::LINEAR => {
            let l = Linear::new_rand(r);
            HttpResponse::Ok().json(l.calculate(data.x))
        }
        FuncType::QUADRATIC => {
            let q = Quadratic::new_rand(r);
            HttpResponse::Ok().json(q.calculate(data.x))
        }
        FuncType::TRIGONOMETRIC_SIN => {
            let t = Trigonometric::new_rand(r, TrigonometricType::SIN);
            HttpResponse::Ok().json(t.calculate(data.x))
        }
        FuncType::TRIGONOMETRIC_COS => {
            let t = Trigonometric::new_rand(r, TrigonometricType::COS);
            HttpResponse::Ok().json(t.calculate(data.x))
        }
    }
}
