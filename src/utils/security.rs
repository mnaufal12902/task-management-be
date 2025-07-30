use actix_web::web;
use argon2::password_hash::{SaltString, rand_core::OsRng};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sqlx::{MySqlPool, Row};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hashed) => hashed.to_string(),
        Err(e) => e.to_string(),
    }
}

pub async fn verify_password(
    pool: &web::Data<MySqlPool>,
    username: &String,
    password: &String,
) -> bool {
    let argon2 = Argon2::default();
    let query = r"SELECT password FROM users WHERE username = ?";

    let result = sqlx::query(query)
        .bind(&username)
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(row)) => {
            let hashed_password: String = row.get("password");

            match PasswordHash::new(&hashed_password) {
                Ok(parsed_password) => argon2
                    .verify_password(password.as_bytes(), &parsed_password)
                    .is_ok(),
                Err(_) => false,
            }
        }
        Ok(None) => false,
        Err(_) => false,
    }
}

