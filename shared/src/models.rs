pub struct User {
    pub uuid:Uuid,
    pub username: String,
    pub created_at: Timestamp,
}

pub struct createUser {
    pub username: String,
}