use actix_web::{HttpRequest, Responder, web};
use sqlx::mysql::MySqlPool;

use crate::{
    models::{
        group::{GroupResponse, GroupRow, UserDetail},
        message::ErrorMessage,
        tasks::{
            CreateTaskRequest, FinishedTaskRequest, GroupTaskStatusResponse, TaskResponse,
            TaskType, UpdateTaskRequest, UserTaskStatusResponse,
        },
        users::{Role, UserResponse},
    },
    utils::{jwt::extract_claims, responder::ApiResponder},
};

// Get All task from database
pub async fn get_all_task(pool: web::Data<MySqlPool>) -> impl Responder {
    let query = "Select id as task_id, course, title, description, task_type, due_date, created_at from tasks";

    let result = sqlx::query_as::<_, TaskResponse>(query)
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(data) => ApiResponder::success(ErrorMessage::Success.to_string(), Some(data)),
        Err(e) => ApiResponder::<()>::handle_error(e),
    }
}

// Get spesific task from database with id task
pub async fn get_task(pool: web::Data<MySqlPool>, id: web::Path<i32>) -> impl Responder {
    let query =
        "Select id, course, title, description, due_date, created_at from tasks where id = ?";

    let result = sqlx::query_as::<_, TaskResponse>(query)
        .bind(*id)
        .fetch_optional(pool.get_ref())
        .await;

    match result {
        Ok(Some(data)) => ApiResponder::success(ErrorMessage::Success.to_string(), Some(data)),
        Ok(None) => ApiResponder::not_found(ErrorMessage::NotFound.to_string(), None::<()>),
        Err(e) => ApiResponder::<()>::handle_error(e),
    }
}

// Create task to database
pub async fn create_task(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    data: web::Json<CreateTaskRequest>,
) -> impl Responder {
    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::has_permission(&claims.role) {
        let task_type = match TaskType::try_from(data.task_type) {
            Ok(t) => t,
            Err(e) => {
                return ApiResponder::bad_request(
                    ErrorMessage::TaskTypeError {
                        details: e.to_owned(),
                    }
                    .to_string(),
                    None::<()>,
                );
            }
        };

        let task_query = r"INSERT INTO tasks (course, title, description, task_type, due_date) VALUES (?, ?, ?, ?, ?)";

        let result = sqlx::query(task_query)
            .bind(&data.course)
            .bind(&data.title)
            .bind(&data.description)
            .bind(task_type as i32)
            .bind(&data.due_date)
            .execute(pool.get_ref())
            .await;

        match result {
            Ok(res) => {
                let inserted_id = res.last_insert_id();
                ApiResponder::success(
                    ErrorMessage::Success.to_string(),
                    Some(TaskResponse {
                        task_id: inserted_id as i32,
                        course: data.course.clone(),
                        title: data.title.clone(),
                        description: data.description.clone(),
                        task_type: task_type as i32,
                        due_date: data.due_date,
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

// Delete task from database with id
pub async fn delete_task(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> impl Responder {
    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::has_permission(&claims.role) {
        let query = "Delete from tasks where id = ?";

        let result = sqlx::query(query).bind(*id).execute(pool.get_ref()).await;

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

// Update task detail
pub async fn update_task(
    pool: web::Data<MySqlPool>,
    req: HttpRequest,
    data_req: web::Json<UpdateTaskRequest>,
) -> impl Responder {
    let claims = match extract_claims(&req).await {
        Ok(claims) => claims,
        Err(e) => return e,
    };

    if Role::has_permission(&claims.role) {
        let query = r"UPDATE tasks SET title = ?, description = ? WHERE id = ?";

        let response = sqlx::query(query)
            .bind(&data_req.title)
            .bind(&data_req.description)
            .bind(&data_req.task_id)
            .execute(pool.get_ref())
            .await;

        match response {
            Ok(_) => ApiResponder::success(ErrorMessage::UpdateDataSuccess.to_string(), None::<()>),
            Err(e) => ApiResponder::<()>::handle_error(e)
        }
    } else {
        ApiResponder::unauthorized(ErrorMessage::InsufficientPermissions.to_string(), None::<()>)
    }
}

// Create finished task to database
pub async fn create_finished_task(
    pool: web::Data<MySqlPool>,
    request: web::Json<FinishedTaskRequest>,
) -> impl Responder {
    let query = r"INSERT INTO finished_user_tasks (task_id, user_id) VALUES (?, ?) ";

    let result = sqlx::query(query)
        .bind(&request.task_id)
        .bind(&request.user_id)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => ApiResponder::success(ErrorMessage::Success.to_string(), None::<()>),
        Err(e) => ApiResponder::<()>::handle_error(e),
    }
}

pub async fn get_task_status(
    pool: web::Data<MySqlPool>,
    task_id: web::Path<i32>,
) -> impl Responder {
    // Query to fetch task details, including task_type
    let task_query = r#"
        SELECT 
            id as task_id, course, title, description, task_type, due_date, created_at 
        FROM tasks 
        WHERE id = ?
    "#;

    let task_info_result = sqlx::query_as::<_, TaskResponse>(task_query)
        .bind(*task_id)
        .fetch_one(pool.get_ref())
        .await;

    match task_info_result {
        Err(e) => ApiResponder::<()>::handle_error(e),
        Ok(task_info) => {
            // Branch based on task_type
            match task_info.task_type {
                0 => {
                    // User task logic
                    let finished_query = r#"
                        SELECT 
                            t.id AS task_id,
                            t.course,
                            t.title,
                            t.description,
                            t.task_type,
                            t.due_date,
                            t.created_at,
                            u.id AS id,
                            u.username,
                            u.name,
                            u.role,
                            u.profile_picture,
                            fu.finished_at
                        FROM tasks t
                        JOIN finished_user_tasks fu ON fu.task_id = t.id
                        JOIN users u ON u.id = fu.user_id
                        WHERE t.id = ?
                    "#;

                    let unfinished_query = r#"
                        SELECT 
                            t.id AS task_id,
                            t.course,
                            t.title,
                            t.description,
                            t.task_type,
                            t.due_date,
                            t.created_at,
                            u.id AS id,
                            u.username,
                            u.name,
                            u.role,
                            u.profile_picture
                        FROM tasks t
                        JOIN users u ON 1=1
                        WHERE t.id = ?
                        AND NOT EXISTS (
                            SELECT 1
                            FROM finished_user_tasks fu
                            WHERE fu.task_id = t.id AND fu.user_id = u.id
                        )
                    "#;

                    let finished_users_result = sqlx::query_as::<_, UserResponse>(finished_query)
                        .bind(*task_id)
                        .fetch_all(pool.get_ref())
                        .await;

                    let unfinished_users_result =
                        sqlx::query_as::<_, UserResponse>(unfinished_query)
                            .bind(*task_id)
                            .fetch_all(pool.get_ref())
                            .await;

                    match (finished_users_result, unfinished_users_result) {
                        (Ok(finished_users), Ok(unfinished_users)) => ApiResponder::success(
                            ErrorMessage::Success.to_string(),
                            Some(UserTaskStatusResponse {
                                task_id: task_info.task_id,
                                course: task_info.course,
                                title: task_info.title,
                                description: task_info.description,
                                task_type: task_info.task_type,
                                due_date: task_info.due_date,
                                finished_users,
                                unfinished_users,
                            }),
                        ),
                        (Err(e), _) | (_, Err(e)) => ApiResponder::<()>::handle_error(e),
                    }
                }
                1 => {
                    // Group task logic
                    let finished_query = r#"
                        SELECT 
                            g.id,
                            g.group_number,
                            g.course,
                            g.created_at
                        FROM tasks t
                        JOIN finished_group_tasks fg ON fg.task_id = t.id
                        JOIN `groups` g ON g.id = fg.group_id
                        WHERE t.id = ?
                    "#;

                    let unfinished_query = r#"
                        SELECT 
                            g.id,
                            g.group_number,
                            g.course,
                            g.created_at
                        FROM tasks t
                        JOIN `groups` g ON g.course = t.course
                        WHERE t.id = ?
                          AND NOT EXISTS (
                              SELECT 1
                              FROM finished_group_tasks fg
                              WHERE fg.task_id = t.id AND fg.group_id = g.id
                          )
                    "#;

                    let finished_group_rows = sqlx::query_as::<_, GroupRow>(finished_query)
                        .bind(*task_id)
                        .fetch_all(pool.get_ref())
                        .await;

                    let unfinished_group_rows = sqlx::query_as::<_, GroupRow>(unfinished_query)
                        .bind(*task_id)
                        .fetch_all(pool.get_ref())
                        .await;

                    match (finished_group_rows, unfinished_group_rows) {
                        (Ok(finished_rows), Ok(unfinished_rows)) => {
                            // Query members for finished groups
                            let mut finished_groups = Vec::new();
                            for group in finished_rows {
                                let members = sqlx::query_as::<_, UserDetail>(
                                    "SELECT u.id, u.username, u.name, u.role, u.profile_picture
                                     FROM users u
                                     JOIN group_members gm ON gm.user_id = u.id
                                     WHERE gm.group_id = ?",
                                )
                                .bind(group.id)
                                .fetch_all(pool.get_ref())
                                .await
                                .unwrap_or_default();

                                finished_groups.push(GroupResponse {
                                    id: group.id,
                                    group_number: group.group_number,
                                    course: group.course,
                                    created_at: group.created_at,
                                    members,
                                });
                            }

                            // Query members for unfinished groups
                            let mut unfinished_groups = Vec::new();
                            for group in unfinished_rows {
                                let members = sqlx::query_as::<_, UserDetail>(
                                    "SELECT u.id, u.username, u.name, u.role, u.profile_picture
                                     FROM users u
                                     JOIN group_members gm ON gm.user_id = u.id
                                     WHERE gm.group_id = ?",
                                )
                                .bind(group.id)
                                .fetch_all(pool.get_ref())
                                .await
                                .unwrap_or_default();

                                unfinished_groups.push(GroupResponse {
                                    id: group.id,
                                    group_number: group.group_number,
                                    course: group.course,
                                    created_at: group.created_at,
                                    members,
                                });
                            }

                            ApiResponder::success(
                                ErrorMessage::Success.to_string(),
                                Some(GroupTaskStatusResponse {
                                    task_id: task_info.task_id,
                                    course: task_info.course,
                                    title: task_info.title,
                                    description: task_info.description,
                                    task_type: task_info.task_type,
                                    due_date: task_info.due_date,
                                    finished_groups,
                                    unfinished_groups,
                                }),
                            )
                        }
                        (Err(e), _) | (_, Err(e)) => ApiResponder::<()>::handle_error(e),
                    }
                }
                _ => ApiResponder::not_found(ErrorMessage::NotFound.to_string(), None::<()>),
            }
        }
    }
}
