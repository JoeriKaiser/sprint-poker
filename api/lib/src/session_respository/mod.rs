use shared::models::{
    CreateSession, Session,
};
use uuid::Uuid;

pub type SessionError = String;
pub type SessionResult<T> = Result<T, SessionError>;

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync + 'static {
    async fn get_session(&self, id: &Uuid) -> SessionResult<Session>;
    async fn create_session(&self, id: &CreateSession) -> SessionResult<Session>;
    async fn delete_session(&self, id: &Uuid) -> SessionResult<Session>;
}