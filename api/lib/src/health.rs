use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub const API_VERSION: &str = "v0.0.1";

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health() -> HttpResponse {
    tracing::info!("healthcheck requested");
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .finish()
}