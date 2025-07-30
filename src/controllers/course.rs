use actix_web::{web, HttpRequest, Responder};
use sqlx::MySqlPool;

use crate::{
    models::{
        course::{AllCourseResponse, CreateCourseRequest},
        message::ErrorMessage,
        users::Role,
    },
    utils::{jwt::extract_claims, responder::ApiResponder},
};

// Get All Subject Task from database
pub async fn get_all_subject(pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query_as::<_, AllCourseResponse>("SELECT course FROM subject")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(data) => ApiResponder::success(ErrorMessage::Success.to_string(), Some(data)),
        Err(e) => ApiResponder::<()>::handle_error(e)
    }
}

pub async fn create_subject(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    data: web::Json<CreateCourseRequest>,
) -> impl Responder {
    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::has_permission(&claims.role) {
        match sqlx::query("INSERT INTO subject (course) VALUES (?)")
            .bind(&data.course)
            .execute(pool.get_ref())
            .await
        {
            Ok(_) => ApiResponder::success(ErrorMessage::Success.to_string(), None::<()>),
            Err(e) => ApiResponder::<()>::handle_error(e)
        }
    } else {
        ApiResponder::unauthorized(
            ErrorMessage::InsufficientPermissions.to_string(),
            None::<()>,
        )
    }
}
