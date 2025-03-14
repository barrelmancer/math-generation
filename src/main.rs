use crate::utils::entities::{GenOptions, FuncType, CalcOptions};
use crate::generation::{linear::Linear, quadratic::Quadratic, trigonometric::{Trigonometric, TrigType}};
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::web::Data;

mod utils;
mod generation;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json("API is up and running!")
}

pub async fn options(data: web::Json<GenOptions>) -> HttpResponse {
    let r = data.range;
    match data.function_type {
        FuncType::LINEAR => {
            let l = Linear::new(r);
            HttpResponse::Ok().json(l.options())
        }
        FuncType::QUADRATIC => {
            let q = Quadratic::new(r);
            HttpResponse::Ok().json(q.options())
        }
        FuncType::TRIGONOMETRIC_SIN => {
            let t = Trigonometric::new(r, TrigType::SIN);
            HttpResponse::Ok().json(t.options())
        }
        FuncType::TRIGONOMETRIC_COS => {
            let t = Trigonometric::new(r, TrigType::COS);
            HttpResponse::Ok().json(t.options())
        }
    }
}
// returns a, b, c ; equation ; calculate
pub async fn get_all_information(data: web::Json<CalcOptions>) -> HttpResponse {
    let r = data.range;
    match data.function_type {
        FuncType::LINEAR => {
            let l = Linear::new(r);
            HttpResponse::Ok().json((l.options(), l.show(), l.calculate(data.x)))
        }
        FuncType::QUADRATIC => {
            let q = Quadratic::new(r);
            HttpResponse::Ok().json((q.options(), q.show(), q.calculate(data.x)))
        }
        FuncType::TRIGONOMETRIC_SIN => {
            let t = Trigonometric::new(r, TrigType::SIN);
            HttpResponse::Ok().json((t.options(), t.show(), t.calculate(data.x)))
        }
        FuncType::TRIGONOMETRIC_COS => {
            let t = Trigonometric::new(r, TrigType::COS);
            HttpResponse::Ok().json((t.options(), t.show(), t.calculate(data.x)))
        }
    }
}

pub async fn equation(data: web::Json<GenOptions>) -> HttpResponse {
    let r = data.range;
    match data.function_type {
        FuncType::LINEAR => {
            let l = Linear::new(r);
            HttpResponse::Ok().json(l.show())
        }
        FuncType::QUADRATIC => {
            let q = Quadratic::new(r);
            HttpResponse::Ok().json(q.show())
        }
        FuncType::TRIGONOMETRIC_SIN => {
            let t = Trigonometric::new(r, TrigType::SIN);
            HttpResponse::Ok().json(t.show())
        }
        FuncType::TRIGONOMETRIC_COS => {
            let t = Trigonometric::new(r, TrigType::COS);
            HttpResponse::Ok().json(t.show())
        }
    }
}

pub async fn calculate(data: web::Json<CalcOptions>) -> HttpResponse {
    let r = data.range;
    match data.function_type {
        FuncType::LINEAR => {
            let l = Linear::new(r);
            HttpResponse::Ok().json(l.calculate(data.x))
        }
        FuncType::QUADRATIC => {
            let q = Quadratic::new(r);
            HttpResponse::Ok().json(q.calculate(data.x))
        }
        FuncType::TRIGONOMETRIC_SIN => {
            let t = Trigonometric::new(r, TrigType::SIN);
            HttpResponse::Ok().json(t.calculate(data.x))
        }
        FuncType::TRIGONOMETRIC_COS => {
            let t = Trigonometric::new(r, TrigType::COS);
            HttpResponse::Ok().json(t.calculate(data.x))
        }
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/equation", web::post().to(equation))
            .route("/calculate", web::post().to(calculate))
            .route("/options", web::post().to(options))
            .route("/get_all_information", web::post().to(get_all_information))
            .route("/health", web::get().to(health_check)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(init)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}