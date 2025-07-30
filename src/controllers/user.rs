use actix_web::{HttpRequest, Responder, web};
use sqlx::mysql::MySqlPool;

use crate::models::message::ErrorMessage;
use crate::models::users::{Role, UpdateUserRequest};
use crate::utils::jwt::extract_claims;
use crate::utils::security::hash_password;
use crate::{
    models::users::{CreateUserRequest, UserResponse},
    utils::responder::ApiResponder,
};

// Get All User From Database
pub async fn get_all_users(pool: web::Data<MySqlPool>, req: HttpRequest) -> impl Responder {
    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::valid_permission(&claims.role) {
        let query = "SELECT id, username, name, role, profile_picture, created_at FROM users";

        let result = sqlx::query_as::<_, UserResponse>(query)
            .fetch_all(pool.get_ref())
            .await;

        match result {
            Ok(data) => ApiResponder::success(ErrorMessage::Success.to_string(), Some(data)),
            Err(e) => ApiResponder::<()>::handle_error(e),
        }
    } else {
        ApiResponder::unauthorized(
            ErrorMessage::InsufficientPermissions.to_string(),
            None::<()>,
        )
    }
}

// Get Spesific User From Database with username
pub async fn get_user(
    pool: web::Data<MySqlPool>,
    path: web::Path<String>,
    req: HttpRequest,
) -> impl Responder {
    let cookies = req.cookies();

    tracing::info!("{:?}", cookies);

    let username = path.into_inner();

    let result = sqlx::query_as::<_, UserResponse>(
        "SELECT id, username, name, role, profile_picture FROM users WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(pool.get_ref())
    .await;

    match result {
        Ok(Some(user)) => ApiResponder::success(ErrorMessage::Success.to_string(), Some(user)),
        Ok(None) => ApiResponder::not_found(ErrorMessage::NotFound.to_string(), None::<()>),
        Err(e) => ApiResponder::<()>::handle_error(e),
    }
}

// Create user to database
pub async fn create_user(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    data: web::Json<CreateUserRequest>,
) -> impl Responder {
    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::has_permission(&claims.role) {
        let encrypted_password = hash_password(&data.password);

        let query = r"INSERT INTO users (username, name, role, password) 
                  VALUES (?, ?, ?, ?)";

        let result = sqlx::query(query)
            .bind(&data.username)
            .bind(&data.name)
            .bind(&data.role.to_string())
            .bind(encrypted_password)
            .execute(pool.get_ref())
            .await;

        match result {
            Ok(_) => ApiResponder::created(ErrorMessage::CreateDataSuccess.to_string(), Some(data)),
            Err(e) => ApiResponder::<()>::handle_error(e),
        }
    } else {
        ApiResponder::unauthorized(
            ErrorMessage::InsufficientPermissions.to_string(),
            None::<()>,
        )
    }
}

// Delete user from database with username
pub async fn delete_user(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let username = path.into_inner();

    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::has_permission(&claims.role) {
        let query = r"Delete from users where username = ?";
        let result = sqlx::query(query)
            .bind(username)
            .execute(pool.get_ref())
            .await;

        match result {
            Ok(res) => {
                if res.rows_affected() == 0 {
                    ApiResponder::not_found(ErrorMessage::NotFound.to_string(), None::<()>)
                } else {
                    ApiResponder::success(ErrorMessage::DeleteSuccess.to_string(), None::<()>)
                }
            }
            Err(e) => ApiResponder::<()>::handle_error(e),
        }
    } else {
        ApiResponder::unauthorized(
            ErrorMessage::InsufficientPermissions.to_string(),
            None::<()>,
        )
    }
}

// Update data user
pub async fn update_data_user(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    data_req: web::Json<UpdateUserRequest>,
) -> impl Responder {
    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::has_permission(&claims.role) || &claims.user_id == &data_req.user_id {
        let query = r"UPDATE users SET name = ?, profile_picture = ? WHERE username = ?";

        let response = sqlx::query(query)
            .bind(&data_req.name)
            .bind(&data_req.profile_picture)
            .bind(&data_req.username)
            .execute(pool.get_ref())
            .await;

        match response {
            Ok(_) => ApiResponder::success(ErrorMessage::UpdateDataSuccess.to_string(), None::<()>),
            Err(e) => ApiResponder::<()>::handle_error(e),
        }
    } else {
        ApiResponder::unauthorized(ErrorMessage::UnAuthorized.to_string(), None::<()>)
    }
}