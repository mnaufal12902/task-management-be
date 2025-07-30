use actix_web::{Responder, web};
use sqlx::MySqlPool;

use crate::{
    models::{
        group_tasks::{GroupTasksResponse, UpdateGroupTasksRequest},
        message::ErrorMessage,
        user_tasks::{FinishedTaskDetail, UnfinishedTaskDetail},
    },
    utils::responder::ApiResponder,
};

// Fetch all tasks with spesific group from database
pub async fn get_group_tasks(
    pool: web::Data<MySqlPool>,
    group_id: web::Path<i32>,
) -> impl Responder {
    let finished_task_query = r#"SELECT g.id as group_id, t.id as task_id, t.title, t.description, 
        t.course, t.due_date, ft.finished_at 
        FROM finished_group_tasks ft
        JOIN groups g ON ft.group = g.id 
        JOIN tasks t ON ft.task_id = t.id 
        WHERE ft.group_id = ?"#;

    let unfinished_task_query = r#" SELECT g.id as grouo_id, t.id as task_id, t.title, t.description, 
        t.course, t.due_date 
        FROM tasks t
        JOIN groups g ON g.id = ?
        LEFT JOIN finished_group_tasks ft ON ft.task_id = t.id AND ft.group_id = g.id
        WHERE ft.task_id IS NULL"#;

    let finished_task_result = sqlx::query_as::<_, FinishedTaskDetail>(finished_task_query)
        .bind(*group_id)
        .fetch_all(pool.get_ref())
        .await;

    let unfinished_task_result = sqlx::query_as::<_, UnfinishedTaskDetail>(unfinished_task_query)
        .bind(*group_id)
        .fetch_all(pool.get_ref())
        .await;

    match (finished_task_result, unfinished_task_result) {
        (Ok(finished_task), Ok(unfinished_task)) => ApiResponder::success(
            ErrorMessage::Success.to_string(),
            Some(GroupTasksResponse {
                group_id: *group_id,
                finished_tasks: finished_task,
                unfinished_tasks: unfinished_task,
            }),
        ),
        (Err(e), _) => ApiResponder::error(
            ErrorMessage::FailedFetchFinishedTask {
                details: e.to_string(),
            }
            .to_string(),
            None::<()>,
        ),
        (Ok(_), Err(e)) => ApiResponder::error(
            ErrorMessage::FailedFetchUnFinishedTask {
                details: e.to_string(),
            }
            .to_string(),
            None::<()>,
        ),
    }
}

// Add finished task for group
pub async fn add_finished_group_task(
    pool: web::Data<MySqlPool>,
    req_data: web::Json<UpdateGroupTasksRequest>,
) -> impl Responder {
    let query = r"INSERT INTO finished_group_tasks (task_id, group_id) VALUES (?, ?)";

    let response = sqlx::query(query)
        .bind(&req_data.task_id)
        .bind(&req_data.group_id)
        .execute(pool.get_ref())
        .await;

    match response {
        Ok(_) => ApiResponder::created(ErrorMessage::CreateDataSuccess.to_string(), None::<()>),
        Err(e) => ApiResponder::<()>::handle_error(e)
    }
}

// Remove finished task for group
pub async fn remove_finished_group_task(pool: web::Data<MySqlPool>, req_data: web::Json<UpdateGroupTasksRequest>) -> impl Responder {
    let query = r"DELETE FROM finished_group_tasks WHERE task_id = ? AND group_id = ?";

    let response = sqlx::query(query)
        .bind(req_data.task_id)
        .bind(req_data.group_id)
        .execute(pool.get_ref())
        .await;

    match response {
        Ok(_) => ApiResponder::success(ErrorMessage::DeleteSuccess.to_string(), None::<()>),
        Err(e) => ApiResponder::<()>::handle_error(e)
    }
}

