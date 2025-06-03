use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ReportMeetings {
    pub month: i32,
    pub count: i32,
}