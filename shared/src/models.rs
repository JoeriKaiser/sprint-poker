use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct User {
    pub uuid:Uuid,
    pub username: String,
    // TODO fix timestamp
    // pub created_at: Timestamp,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateUser {
    pub username: String,
}