use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ExperienceReport {
    pub full_name: String,
    pub experience: i32,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct RetiredReport {
    pub full_name: String,
    pub birthday: NaiveDate,
}