use actix_web::{Responder, web};
use sqlx::{Row, mysql::MySqlPool};

use crate::{
    models::{
        message::ErrorMessage,
        user_tasks::{
            FinishedTaskDetail, UnfinishedTaskDetail, UpdateUserTasksRequest, UserTasksResponse,
        },
    },
    utils::responder::ApiResponder,
};

pub async fn get_user_tasks(pool: web::Data<MySqlPool>, user_id: web::Path<i32>) -> impl Responder {
    let group_rows = match sqlx::query("SELECT group_id FROM group_members WHERE user_id = ?")
        .bind(*user_id)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(rows) => rows,
        Err(err) => return ApiResponder::<()>::handle_error(err),
    };

    let user_group_ids: Vec<i32> = group_rows
        .into_iter()
        .map(|r| r.get::<i32, _>("group_id"))
        .collect();

    let finished_task_query = r#"
        SELECT u.id as user_id, t.id as task_id, t.title, t.description, 
               t.course, t.due_date, ft.finished_at 
        FROM finished_user_tasks ft
        JOIN users u ON ft.user_id = u.id 
        JOIN tasks t ON ft.task_id = t.id 
        WHERE ft.user_id = ?
    "#;

    let finished_task_result = sqlx::query_as::<_, FinishedTaskDetail>(finished_task_query)
        .bind(*user_id)
        .fetch_all(pool.get_ref())
        .await;

    let mut group_finished_tasks: Vec<FinishedTaskDetail> = Vec::new();

    if !user_group_ids.is_empty() {
        let mut group_query = String::from(
            r#"
            SELECT NULL as user_id, t.id as task_id, t.title, t.description,
                   t.course, t.due_date, fg.finished_at
            FROM finished_group_tasks fg
            JOIN tasks t ON fg.task_id = t.id
            WHERE fg.group_id IN "#,
        );

        group_query.push_str(
            &user_group_ids
                .iter()
                .enumerate()
                .map(|(i, _)| if i == 0 { "(?" } else { ", ?" })
                .collect::<String>(),
        );
        group_query.push(')');

        let mut q = sqlx::query_as::<_, FinishedTaskDetail>(&group_query);
        for gid in &user_group_ids {
            q = q.bind(gid);
        }

        group_finished_tasks = match q.fetch_all(pool.get_ref()).await {
            Ok(result) => result,
            Err(err) => return ApiResponder::<()>::handle_error(err),
        };
    }

    let mut unfinished_query = String::from(
        r#"
        SELECT u.id as user_id, t.id as task_id, t.title, t.description, 
               t.course, t.due_date 
        FROM tasks t
        JOIN users u ON u.id = ?
        LEFT JOIN finished_user_tasks fut ON fut.task_id = t.id AND fut.user_id = u.id
        LEFT JOIN finished_group_tasks fgt ON fgt.task_id = t.id
    "#,
    );

    if !user_group_ids.is_empty() {
        unfinished_query.push_str(" AND fgt.group_id IN (");
        unfinished_query.push_str(
            &user_group_ids
                .iter()
                .enumerate()
                .map(|(i, _)| if i == 0 { "?" } else { ", ?" })
                .collect::<String>(),
        );
        unfinished_query.push(')');
    }

    unfinished_query.push_str(" WHERE fut.task_id IS NULL");

    if !user_group_ids.is_empty() {
        unfinished_query.push_str(" AND fgt.task_id IS NULL");
    }

    let mut q = sqlx::query_as::<_, UnfinishedTaskDetail>(&unfinished_query);
    q = q.bind(*user_id);

    for gid in &user_group_ids {
        q = q.bind(gid);
    }

    let unfinished_task_result = q.fetch_all(pool.get_ref()).await;

    match (finished_task_result, unfinished_task_result) {
        (Ok(mut finished_by_user), Ok(unfinished_tasks)) => {
            finished_by_user.extend(group_finished_tasks);

            ApiResponder::success(
                ErrorMessage::Success.to_string(),
                Some(UserTasksResponse {
                    user_id: *user_id,
                    finished_tasks: finished_by_user,
                    unfinished_tasks,
                }),
            )
        }
        (Err(e), _) => ApiResponder::<()>::handle_error(e),
        (Ok(_), Err(e)) => ApiResponder::<()>::handle_error(e),
    }
}

// Add finished task for user
pub async fn add_finished_user_task(
    pool: web::Data<MySqlPool>,
    req_data: web::Json<UpdateUserTasksRequest>,
) -> impl Responder {
    let query = r"INSERT INTO finished_user_tasks (task_id, user_id) VALUES (?, ?)";

    let response = sqlx::query(query)
        .bind(req_data.task_id)
        .bind(req_data.user_id)
        .execute(pool.get_ref())
        .await;

    match response {
        Ok(_) => ApiResponder::created(ErrorMessage::CreateDataSuccess.to_string(), None::<()>),
        Err(e) => ApiResponder::<()>::handle_error(e),
    }
}

// Remove finished task for user
pub async fn remove_finished_user_task(
    pool: web::Data<MySqlPool>,
    req_data: web::Json<UpdateUserTasksRequest>,
) -> impl Responder {
    let query = r"DELETE FROM finished_user_tasks WHERE task_id = ? AND user_id = ?";

    let response = sqlx::query(query)
        .bind(req_data.task_id)
        .bind(req_data.user_id)
        .execute(pool.get_ref())
        .await;

    match response {
        Ok(_) => ApiResponder::success(ErrorMessage::DeleteSuccess.to_string(), None::<()>),
        Err(e) => ApiResponder::<()>::handle_error(e),
    }
}
