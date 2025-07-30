use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub course: String
}

#[derive(Serialize, Deserialize)]
pub struct CreateGroupResponse {
    pub id: i32,
    pub group_number: i32,
    pub course: String,
    pub members: Vec<UserDetail>,
}

#[derive(Serialize, Deserialize)] 
pub struct AddMembersRequest {
    pub users: Vec<i32>,
}

#[derive(Serialize, Deserialize)] 
pub struct RemoveMemberRequest {
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct GroupResponse {
    pub id: i32,
    pub group_number: i32,
    pub course: String,
    pub members: Vec<UserDetail>,
    pub created_at: NaiveDateTime
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct GroupRow {
    pub id: i32,
    pub group_number: i32,
    pub course: String,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct UserDetail {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub role: String,
    pub profile_picture: Option<String>
}

