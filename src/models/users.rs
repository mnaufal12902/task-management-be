use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub name: String,
    pub role: Role,
    pub password: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub role: String,
    pub profile_picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "PascalCase")]
pub enum Role {
    Anggota,
    Ketua,
    Sekretaris,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub user_id: i32,
    pub username: String,
    pub name: String,
    pub profile_picture: Option<String>,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Role::Anggota => "Anggota",
            Role::Ketua => "Ketua",
            Role::Sekretaris => "Sekretaris",
        };
        write!(f, "{}", s)
    }
}

impl Role {
    pub fn has_permission(&self) -> bool {
        match self {
            Role::Ketua | Role::Sekretaris => true,
            _ => false,
        }
    }

    pub fn valid_permission(&self) -> bool {
        match self {
            Role::Ketua | Role::Sekretaris | Role::Anggota => true,
            _ => false,
        }
    }
}
