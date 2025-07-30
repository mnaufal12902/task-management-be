use serde::{Deserialize, Serialize};

use crate::models::user_tasks::{FinishedTaskDetail, UnfinishedTaskDetail};

#[derive(Serialize, Deserialize)]
pub struct UpdateGroupTasksRequest {
    pub task_id: i32,
    pub group_id: i32,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct GroupTasksResponse {
    pub group_id: i32,
    pub finished_tasks: Vec<FinishedTaskDetail>,
    pub unfinished_tasks: Vec<UnfinishedTaskDetail>,
}
