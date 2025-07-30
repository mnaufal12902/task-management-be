use std::env;

use actix_web::{HttpRequest, HttpResponse, cookie::Cookie, web};
use chrono::{DateTime, Duration, Utc};
use sqlx::{MySqlPool, Row};
use uuid::Uuid;

use crate::{
    models::message::ErrorMessage,
    utils::{
        jwt::{create_token},
        responder::ApiResponder,
    },
};

pub async fn renew_access_token(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let cookies = req.cookies()?;

    tracing::info!("{:?}", cookies);

    if let Some(refresh_cookie) = cookies.iter().find(|c| c.name() == "refresh_token") {
        let refresh_token = refresh_cookie.value();

        if let Some(user_id) = get_user_id_from_refresh_token(&pool, &refresh_token).await {
            let is_valid = validation_refresh_token(&pool, user_id, refresh_token).await;

            if is_valid {
                {
                    let new_token =
                        create_token(&pool, &env::var("SECRET_KEY")?, &user_id.to_string()).await?;

                    let access_cookie =
                        Cookie::build("access_token", new_token.access_token.clone())
                            .http_only(true)
                            .secure(false)
                            .path("/")
                            .finish();

                    let refresh_cookie =
                        Cookie::build("refresh_token", new_token.refresh_token.clone())
                            .http_only(true)
                            .secure(false)
                            .path("/")
                            .finish();

                    let cookies = vec![access_cookie, refresh_cookie];

                    return Ok(ApiResponder::success_with_cookie(
                        ErrorMessage::Success.to_string(),
                        Some(serde_json::json!({ "token": new_token.access_token })),
                        cookies,
                    ));
                };
            } else {
                return Ok(ApiResponder::unauthorized(
                    ErrorMessage::RefreshTokenInvalid.to_string(),
                    None::<()>,
                ));
            }
        } else {
            return Ok(ApiResponder::unauthorized(
                ErrorMessage::RefreshTokenInvalid.to_string(),
                None::<()>,
            ));
        };
    }

    Ok(ApiResponder::bad_request(
        ErrorMessage::RefreshTokenInvalid.to_string(),
        None::<()>,
    ))
}

pub async fn get_or_create_refresh_token(
    pool: &web::Data<MySqlPool>,
    user_id: i32,
    session_id: String,
) -> Result<String, sqlx::Error> {
    if let Some(token) = check_refresh_token(&pool, user_id).await? {
        Ok(token)
    } else {
        create_refresh_token(&pool, user_id, session_id).await
    }
}

pub async fn create_refresh_token(
    pool: &web::Data<MySqlPool>,
    user_id: i32,
    session_id: String,
) -> Result<String, sqlx::Error> {
    let query = r"INSERT INTO sessions (session_id, user_id, refresh_token, expires_at) 
                  VALUES (?, ?, ?, ?)";

    let refresh_token = Uuid::new_v4().to_string();
    let created_at = chrono::Utc::now();
    let expires_at = created_at + Duration::hours(720); // 30 days

    sqlx::query(query)
        .bind(&session_id)
        .bind(user_id)
        .bind(&refresh_token)
        .bind(&expires_at)
        .execute(pool.get_ref())
        .await?;

    Ok(refresh_token)
}

pub async fn check_refresh_token(
    pool: &MySqlPool,
    user_id: i32,
) -> Result<Option<String>, sqlx::Error> {
    let query = r"SELECT expires_at, refresh_token FROM sessions WHERE user_id = ?";

    let result = sqlx::query(query)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

    if let Some(row) = result {
        let expires_at: DateTime<Utc> = row.get("expires_at");
        let token: String = row.get("refresh_token");

        if expires_at < chrono::Utc::now() {
            Ok(None)
        } else {
            Ok(Some(token))
        }
    } else {
        Ok(None)
    }
}

pub async fn validation_refresh_token(
    pool: &web::Data<MySqlPool>,
    user_id: i32,
    refresh_token: &str,
) -> bool {
    let query = r"SELECT refresh_token from sessions WHERE user_id = ?";

    let result = sqlx::query(query)
        .bind(&user_id)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(row) => {
            let refresh_token_db: String = row.get("refresh_token");

            if refresh_token_db == refresh_token {
                true
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

pub async fn get_user_id_from_refresh_token(
    pool: &web::Data<MySqlPool>,
    refresh_token: &str,
) -> Option<i32> {
    let query = r"Select user_id from sessions Where refresh_token = ?";

    let result = sqlx::query(query)
        .bind(&refresh_token)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(row) => {
            let user_id: i32 = row.get("user_id");
            Some(user_id)
        }
        Err(_) => None,
    }
}
