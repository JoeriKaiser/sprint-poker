use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

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
        cfg.app_data(pool).configure(
            api_lib::health::service
        );
    };

    tracing::info!("Database succesfully started !");

    Ok(config.into())
}
