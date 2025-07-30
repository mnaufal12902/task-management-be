use std::env;

use actix_web::{cookie::{time::Duration, Cookie}, web, Responder};
use sqlx::MySqlPool;

use crate::{
    models::{auth::LoginRequest, message::ErrorMessage},
    utils::{jwt::create_token, responder::ApiResponder, security::verify_password},
};

pub async fn login_handler(
    pool: web::Data<MySqlPool>,
    data: web::Json<LoginRequest>,
) -> impl Responder {
    let login = data.into_inner();

    let is_valid = verify_password(&pool, &login.username, &login.password).await;

    let token = create_token(&pool, &env::var("SECRET_KEY").unwrap(), &login.username).await;

    if is_valid {
        match token {
            Ok(token) => {
                let access_cookie = Cookie::build("access_token", token.access_token.clone())
                    .http_only(true)
                    .secure(false) 
                    .path("/")
                    .finish();

                let refresh_cookie = Cookie::build("refresh_token", token.refresh_token.clone())
                    .http_only(true)
                    .secure(false)
                    .path("/")
                    .finish();

                let cookies = vec![access_cookie, refresh_cookie];

                ApiResponder::success_with_cookie(
                    ErrorMessage::LoginSuccess.to_string(),
                    Some(serde_json::json!({ "token": token.access_token })),
                    cookies,
                )
            }
            Err(e) => ApiResponder::unauthorized(
                ErrorMessage::TokenGenerateFailed {
                    details: e.to_string(),
                }
                .to_string(),
                None::<()>,
            ),
        }
    } else {
        ApiResponder::unauthorized(ErrorMessage::LoginInvalid.to_string(), None::<()>)
    }
}

pub async fn logout_handler() -> impl Responder {
    let access_cookie = Cookie::build("access_token", "")
        .path("/")
        .http_only(true)
        .max_age(Duration::seconds(0))
        .finish();

    let refresh_cookie = Cookie::build("refresh_token", "")
        .path("/")
        .http_only(true)
        .max_age(Duration::seconds(0))
        .finish();

    let cookies = vec![access_cookie, refresh_cookie];

    ApiResponder::success_with_cookie(
        ErrorMessage::LogoutSuccess.to_string(),
        Some(serde_json::json!({
            "message": "Successfully logged out"
        })),
        cookies,
    )
}
