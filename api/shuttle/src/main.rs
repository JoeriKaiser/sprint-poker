use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> Result<String, actix_web::Error>{
    tracing::info!("Getting version !");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;
    
    match result {
        Ok(version) => Ok(version),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    tracing::info!("Starting Database!");
    pool.execute(include_str!("../../db/schema.sql"))
    .await
    .map_err(CustomError::new)?;
    let pool = actix_web::web::Data::new(pool);
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool).service(hello_world).service(version);
    };
    tracing::info!("Database succesfully started !");

    Ok(config.into())
}
