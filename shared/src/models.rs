use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct Session {
    pub uuid: Uuid,
    pub session_name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateUser {
    pub username: String,
}

pub struct CreateSession {
    pub session_name: String,
}