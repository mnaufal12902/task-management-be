use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::models::{group::GroupResponse, users::UserResponse};

#[derive(Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub course: String,
    pub title: String,
    pub description: String,
    pub task_type: i32,
    pub due_date: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    pub task_id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct TaskResponse {
    pub task_id: i32,
    pub course: String,
    pub title: String,
    pub description: String,
    pub task_type: i32,
    pub due_date: NaiveDateTime,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct FinishedTaskRequest {
    pub user_id: i32,
    pub task_id: i32,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct FinishedTaskResponse {
    pub user_id: i32,
    pub task_id: i32,
    pub finished_at: NaiveDateTime,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct UserTaskStatus {
    pub user_id: i32,
    pub username: String,
    pub name: String,
    pub task_finished: bool,
    pub finished_at: Option<NaiveDateTime>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct UserTaskStatusResponse {
    pub task_id: i32,
    pub course: String,
    pub title: String,
    pub description: String,
    pub task_type: i32,
    pub due_date: NaiveDateTime,
    pub finished_users: Vec<UserResponse>,
    pub unfinished_users: Vec<UserResponse>,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct GroupTaskStatusResponse {
    pub task_id: i32,
    pub course: String,
    pub title: String,
    pub description: String,
    pub task_type: i32,
    pub due_date: NaiveDateTime,
    pub finished_groups: Vec<GroupResponse>,
    pub unfinished_groups: Vec<GroupResponse>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(i32)]
pub enum TaskType {
    Individual = 0,
    Group = 1,
}

impl TryFrom<i32> for TaskType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TaskType::Individual),
            1 => Ok(TaskType::Group),
            _ => Err("Invalid task type"),
        }
    }
}

impl From<TaskType> for i32 {
    fn from(t: TaskType) -> Self {
        t as i32
    }
}
