use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use chrono::{NaiveDate, NaiveDateTime};
use crate::types::custom_serde;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Members {
    pub id: Uuid,
    pub full_name: String,
    #[serde(with = "custom_serde")]
    pub birthday: NaiveDate,
    pub experience: i32,
    pub phone: String,
    pub is_chairman: bool,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub updated_at: NaiveDateTime,
}