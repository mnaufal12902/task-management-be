use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)] 
pub struct AllCourseResponse {
    pub course: String
}

#[derive(Serialize, Deserialize)] 
pub struct CreateCourseRequest {
    pub course: String
}