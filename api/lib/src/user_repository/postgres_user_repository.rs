use async_trait;
use shared::models::{CreateUser, User};

use super::{UserRepository, UserResult};

pub struct PostgresUserRepository {
    pool: sqlx::PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepository {
    async fn get_users(&self) -> UserResult<Vec<User>> {
        sqlx::query_as::<_, User>(
            r#"
        SELECT id, username, created_At
        FROM users
      "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn get_user(&self, user_id: &uuid::Uuid) -> UserResult<User> {
        sqlx::query_as::<_, User>(
            r#"
        SELECT id, username, created_At
        FROM users
        WHERE id = $1
      "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn create_user(&self, create_user: &CreateUser) -> UserResult<User> {
        sqlx::query_as::<_, User>(
            r#"
        INSERT INTO users (username)
        VALUES ($1, $2)
        RETURNING id, username, created_at
      "#,
        )
        .bind(&create_user.username)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn update_user(&self, user: &User) -> UserResult<User> {
        sqlx::query_as::<_, User>(
            r#"
        UPDATE users
        SET username = $2
        WHERE id = $1
        RETURNING id, username, created_at
      "#,
        )
        .bind(&user.username)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }

    async fn delete_user(&self, user_id: &uuid::Uuid) -> UserResult<uuid::Uuid> {
        sqlx::query_scalar::<_, uuid::Uuid>(
            r#"
        DELETE FROM users
        WHERE id = $1
        RETURNING id
      "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
