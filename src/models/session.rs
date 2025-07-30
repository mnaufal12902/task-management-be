use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub session_id: String,
    pub user_id: i32,
    pub refresh_token: String,
    pub user_agent: String,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeSessionRequest {
    pub session_id: String,
    pub is_revoke: bool
}