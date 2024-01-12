use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("v1/votes")
            .route("/{votes_id}", web::get().to(get))
            .route("", web::post().to(post))
            .route("/{votes_id}", web::delete().to(delete))
    );
}

async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn post() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete() -> HttpResponse {
    HttpResponse::Ok().finish()
}