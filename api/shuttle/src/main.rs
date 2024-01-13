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

    let user_repository = api_lib::user_repository::PostgresUserRepository::new(pool);
    let user_repository = actix_web::web::Data::new(user_repository);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(user_repository)
        .configure(api_lib::health::service)
        .configure(api_lib::users::service);
    };

    tracing::info!("Database succesfully started !");

    Ok(config.into())
}
