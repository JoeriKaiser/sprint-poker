use chrono::format::Numeric;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[cfg(feature = "backend")]
use sqlx::FromRow;

#[cfg_attr(feature = "backend", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct User {
    pub uuid: Uuid,
    pub username: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Session {
    pub uuid: Uuid,
    pub session_name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Ticket {
    pub uuid: Uuid,
    pub username: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Vote {
    pub uuid: Uuid,
    pub value: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "backend", derive(FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateSession {
    pub session_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateTicket {
    pub ticket_name: String,
}

pub struct CreateVote {
    pub vote_value: Numeric,
}