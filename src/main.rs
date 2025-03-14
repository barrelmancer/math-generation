use crate::api_scopes::{
    generation::{calculate, equation, get_all_information, options},
    health::health_check,
    math::scope_derivative,
};
use crate::functions::{
    linear::Linear,
    operations::Operations,
    quadratic::Quadratic,
    trigonometric::{Trigonometric, TrigonometricType},
};

use crate::utils::mathematical_functions::derivative;

use crate::utils::entities::{CalcOptions, FuncType, GenOptions};

use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, web};

mod api_scopes;
mod functions;
mod utils;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Health check
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/random")
                    .route("/equation", web::post().to(equation))
                    .route("/calculate", web::post().to(calculate))
                    .route("/options", web::post().to(options))
                    .route("/get_all_information", web::post().to(get_all_information)),
            )
            .service(web::scope("/math").route("/derivative", web::post().to(scope_derivative))),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(init))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
