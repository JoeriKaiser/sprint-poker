use shared::models::{
    CreateVote, Vote,
};
use uuid::Uuid;

pub type VoteError = String;
pub type VoteResult<T> = Result<T, VoteError>;

#[async_trait::async_trait]
pub trait VoteRepository: Send + Sync + 'static {
    async fn get_session(&self, id: &Uuid) -> VoteResult<Vote>;
    async fn create_session(&self, id: &CreateVote) -> VoteResult<Vote>;
    async fn delete_session(&self, id: &Uuid) -> VoteResult<Vote>;
}