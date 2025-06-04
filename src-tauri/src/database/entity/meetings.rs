use sqlx::types::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{NaiveDate, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Meetings {
    pub id: Uuid,
    pub date: NaiveDate,
    pub duration: i32,
    pub commission_id: Uuid,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub updated_at: NaiveDateTime,
}
