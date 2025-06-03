use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{NaiveDate, NaiveDateTime};
use sqlx::types::Uuid;
use crate::types::custom_serde;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Organizers {
    pub id: Uuid,
    pub full_name: String,
    pub phone: String,
    #[serde(with = "custom_serde")]
    pub birthday: NaiveDate,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub updated_at: NaiveDateTime,
}