use sqlx::types::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct CommissionMembers {
    pub id: Uuid,
    pub member_id: Uuid,
    pub commission_id: Uuid,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub updated_at: NaiveDateTime,
}