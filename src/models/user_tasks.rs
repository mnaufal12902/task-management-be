use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] 
pub struct UpdateUserTasksRequest {
    pub task_id: i32,
    pub user_id: i32,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct UserTasksResponse {
    pub user_id: i32,
    pub finished_tasks: Vec<FinishedTaskDetail>,
    pub unfinished_tasks: Vec<UnfinishedTaskDetail>
}

#[derive(Serialize, sqlx::FromRow, Debug)] 
pub struct FinishedTaskDetail {
    pub task_id: i32,
    pub title: String,
    pub description: String,
    pub course: String,
    pub due_date: NaiveDateTime,
    pub finished_at: NaiveDateTime,
}

#[derive(Serialize, sqlx::FromRow)] 
pub struct UnfinishedTaskDetail {
    pub task_id: i32,
    pub title: String,
    pub description: String,
    pub course: String,
    pub due_date: NaiveDateTime,
}

