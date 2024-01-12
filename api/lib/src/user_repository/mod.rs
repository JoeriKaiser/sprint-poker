use shared::models::{
    CreateUser, User,
    CreateSession, Session,
    CreateTicket, Ticket
};
use uuid::Uuid;

// USER
pub type UserError = String;
pub type UserResult<T> = Result<T, UserError>;

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn get_users(&self) -> UserResult<Vec<User>>;
    async fn get_user(&self, id: &Uuid) -> UserResult<User>;
    async fn create_user(&self, id: &CreateUser) -> UserResult<User>;
    async fn update_user(&self, id: &User) -> UserResult<User>;
    async fn delete_user(&self, id: &Uuid) -> UserResult<User>;
}

// SESSION
pub type SessionError = String;
pub type SessionResult<T> = Result<T, SessionError>;

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync + 'static {
    async fn get_session(&self, id: &Uuid) -> SessionResult<Session>;
    async fn create_session(&self, id: &CreateSession) -> SessionResult<Session>;
    async fn delete_session(&self, id: &Uuid) -> SessionResult<Session>;
}

// TICKET
pub type TicketError = String;
pub type TicketResult<T> = Result<T, TicketError>;

#[async_trait::async_trait]
pub trait TicketRepository: Send + Sync + 'static {
    async fn get_users(&self) -> TicketResult<Vec<Ticket>>;
    async fn get_user(&self, id: &Uuid) -> TicketResult<Ticket>;
    async fn create_user(&self, id: &CreateTicket) -> TicketResult<Ticket>;
    async fn update_user(&self, id: &Ticket) -> TicketResult<Ticket>;
    async fn delete_user(&self, id: &Uuid) -> TicketResult<Ticket>;
}