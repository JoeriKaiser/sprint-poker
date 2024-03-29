pub use postgres_user_repository::PostgresUserRepository;

use shared::models::{
    CreateUser, User,
};
use uuid::Uuid;

mod postgres_user_repository;

pub type UserError = String;
pub type UserResult<T> = Result<T, UserError>;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn get_users(&self) -> UserResult<Vec<User>>;
    async fn get_user(&self, id: &Uuid) -> UserResult<User>;
    async fn create_user(&self, id: &CreateUser) -> UserResult<User>;
    async fn update_user(&self, id: &User) -> UserResult<User>;
    async fn delete_user(&self, id: &Uuid) -> UserResult<uuid::Uuid>;
}