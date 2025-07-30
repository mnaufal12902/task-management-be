use std::collections::HashMap;

use actix_web::{HttpRequest, Responder, web};
use chrono::NaiveDateTime;
use sqlx::MySqlPool;

use crate::{
    models::{
        group::{
            AddMembersRequest, CreateGroupRequest, CreateGroupResponse, GroupResponse,
            RemoveMemberRequest, UserDetail,
        },
        message::ErrorMessage,
        users::Role,
    },
    utils::{jwt::extract_claims, responder::ApiResponder},
};

// Create Group to database
pub async fn create_grup(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    request: web::Json<CreateGroupRequest>,
) -> impl Responder {
    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::has_permission(&claims.role) {
        let max_query = r"SELECT MAX(group_number) as last_number FROM `groups` WHERE course = ?";

        let last_number: Option<i32> = match sqlx::query_scalar(max_query)
            .bind(&request.course)
            .fetch_one(pool.get_ref())
            .await
        {
            Ok(num) => num,
            Err(_) => Some(0),
        };

        let next_group_number = last_number.unwrap_or(0) + 1;

        let insert_query = r"INSERT INTO `groups` (course, group_number) VALUES (?, ?)";

        let result = sqlx::query(insert_query)
            .bind(&request.course)
            .bind(next_group_number)
            .execute(pool.get_ref())
            .await;

        match result {
            Ok(res) => {
                let inserted_id = res.last_insert_id();
                ApiResponder::success(
                    ErrorMessage::Success.to_string(),
                    Some(CreateGroupResponse {
                        id: inserted_id as i32,
                        course: request.course.clone(),
                        members: Vec::new(),
                        group_number: next_group_number,
                    }),
                )
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

// Add multi user to group
pub async fn add_member_to_group(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    group_id: web::Path<i32>,
    users_id: web::Json<AddMembersRequest>,
) -> impl Responder {
    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::has_permission(&claims.role) {
        let mut query = String::from("INSERT INTO group_members (group_id, user_id) VALUES ");
        let group_id = group_id.into_inner();

        query.push_str(
            &users_id
                .users
                .iter()
                .enumerate()
                .map(|(i, _)| if i > 0 { ", (?, ?)" } else { "(?, ?)" })
                .collect::<String>(),
        );

        let mut sql_query = sqlx::query(&query);

        for user_id in &users_id.users {
            sql_query = sql_query.bind(group_id).bind(user_id);
        }

        let result = sql_query.execute(pool.get_ref()).await;

        match result {
            Ok(_) => ApiResponder::success(ErrorMessage::Success.to_string(), None::<()>),
            Err(e) => ApiResponder::<()>::handle_error(e),
        }
    } else {
        ApiResponder::unauthorized(
            ErrorMessage::InsufficientPermissions.to_string(),
            None::<()>,
        )
    }
}

// Delete spesific members to from group
pub async fn remove_member_from_group(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    group_id: web::Path<i32>,
    request: web::Json<RemoveMemberRequest>,
) -> impl Responder {
    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::has_permission(&claims.role) {
        let query = r"Delete from group_members WHERE user_id = ? && group_id = ?";

        let result = sqlx::query(query)
            .bind(request.user_id)
            .bind(*group_id)
            .execute(pool.get_ref())
            .await;

        match result {
            Ok(e) => {
                if e.rows_affected() == 0 {
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

// Get all groups with their members
pub async fn get_all_groups(pool: web::Data<MySqlPool>) -> impl Responder {
    let query = r#"
        SELECT g.id, g.group_number, g.course, g.created_at,
               gm.user_id, u.username, u.name, u.role, u.profile_picture
        FROM `groups` g
        LEFT JOIN group_members gm ON g.id = gm.group_id
        LEFT JOIN users u ON gm.user_id = u.id
    "#;

    let rows = sqlx::query_as::<
        _,
        (
            i32,
            i32,
            String,
            NaiveDateTime,
            Option<i32>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    >(query)
    .fetch_all(pool.get_ref())
    .await;

    match rows {
        Ok(data) => {
            let mut group_map: HashMap<i32, GroupResponse> = HashMap::new();

            for (
                id,
                group_number,
                course,
                created_at,
                user_id_opt,
                username_opt,
                name_opt,
                role_opt,
                profile_picture_opt,
            ) in data
            {
                let entry = group_map.entry(id).or_insert_with(|| GroupResponse {
                    id,
                    group_number,
                    course,
                    created_at,
                    members: Vec::new(),
                });

                if let (Some(user_id), Some(username), Some(name), Some(role)) =
                    (user_id_opt, username_opt, name_opt, role_opt)
                {
                    entry.members.push(UserDetail {
                        id: user_id,
                        username,
                        name,
                        role,
                        profile_picture: profile_picture_opt,
                    });
                }
            }

            let groups: Vec<GroupResponse> = group_map.into_values().collect();

            ApiResponder::success(ErrorMessage::Success.to_string(), Some(groups))
        }
        Err(e) => ApiResponder::<()>::handle_error(e),
    }
}

// Delete groups from database
pub async fn delete_group(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    group_id: web::Path<i32>,
) -> impl Responder {
    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::has_permission(&claims.role) {
        let query = r"DELETE FROM `groups` where id = ?";

        let result = sqlx::query(query)
            .bind(*group_id)
            .execute(pool.get_ref())
            .await;

        match result {
            Ok(e) => {
                if e.rows_affected() == 0 {
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
