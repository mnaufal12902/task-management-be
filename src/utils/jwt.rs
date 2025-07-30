use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::{HttpRequest, HttpResponse, web};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use nanoid::nanoid;
use sqlx::{MySqlPool, Row};

use crate::{
    controllers::session::get_or_create_refresh_token,
    models::{
        auth::{Claims, Token},
        message::ErrorMessage,
        users::Role,
    },
};

use super::responder::ApiResponder;

pub async fn create_token(
    pool: &web::Data<MySqlPool>,
    secret: &String,
    username: &String,
) -> Result<Token, Box<dyn std::error::Error>> {
    let expiration = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() + 1 * 60 * 60; // 1 hours in seconds
    let query = r"SELECT id, role FROM users WHERE username = ?";

    let result = sqlx::query(query)
        .bind(&username)
        .fetch_optional(pool.get_ref())
        .await;

    let user_role: Role;
    let mut user_id: i32 = 0;

    match result {
        Ok(Some(row)) => {
            let role_str: String = row.get("role");
            user_id = row.get("id");

            user_role = match role_str.as_str() {
                "Anggota" => Role::Anggota,
                "Ketua" => Role::Ketua,
                "Sekretaris" => Role::Sekretaris,
                _ => Role::Anggota,
            };
        }
        Ok(None) => {
            user_role = Role::Anggota;
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    let claims = Claims {
        sub: username.to_owned(),
        user_id,
        role: user_role,
        exp: expiration as usize,
    };

    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    let session_id = nanoid!(20);

    let access_token = encode(&header, &claims, &encoding_key)?;
    let refresh_token = get_or_create_refresh_token(&pool, user_id, session_id).await?;

    Ok(Token {
        access_token,
        refresh_token,
    })
}

pub fn decode_token(token: &str) -> Result<Claims, String> {
    let decoding_key = DecodingKey::from_secret(env::var("SECRET_KEY").unwrap().as_bytes());

    match decode::<Claims>(&token, &decoding_key, &Validation::default()) {
        Ok(token_data) => Ok(token_data.claims),
        Err(e) => Err(ErrorMessage::TokenDecodeError {
            details: e.to_string(),
        }
        .to_string()),
    }
}

pub async fn extract_claims(req: &HttpRequest) -> Result<Claims, HttpResponse> {
    let auth_header = req.headers().get("Authorization").ok_or_else(|| {
        ApiResponder::unauthorized(ErrorMessage::NoAuthHeader.to_string(), None::<()>)
    })?;

    let auth_str = auth_header.to_str().map_err(|_| {
        ApiResponder::unauthorized(ErrorMessage::InvalidAuthHeader.to_string(), None::<()>)
    })?;

    if !auth_str.starts_with("Bearer ") {
        return Err(ApiResponder::unauthorized(
            ErrorMessage::InvalidAuthScheme.to_string(),
            None::<()>,
        ));
    }

    let token = &auth_str[7..];
    decode_token(token).map_err(|e| ApiResponder::unauthorized(e, None::<()>))
}
