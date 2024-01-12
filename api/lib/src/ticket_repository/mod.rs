use shared::models::{
    CreateTicket, Ticket,
};
use uuid::Uuid;

pub type TicketError = String;
pub type TicketResult<T> = Result<T, TicketError>;

#[async_trait::async_trait]
pub trait TicketRepository: Send + Sync + 'static {
    async fn get_ticket(&self) -> TicketResult<Vec<Ticket>>;
    async fn get_ticket(&self, id: &Uuid) -> TicketResult<Ticket>;
    async fn create_ticket(&self, id: &CreateTicket) -> TicketResult<Ticket>;
    async fn update_ticket(&self, id: &Ticket) -> TicketResult<Ticket>;
    async fn delete_ticket(&self, id: &Uuid) -> TicketResult<Ticket>;
}