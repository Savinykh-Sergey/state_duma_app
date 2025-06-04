use sqlx::types::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

use crate::database::enums::commission_referral::CommissionDirection;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Commissions {
    pub id: Uuid,
    pub direction: CommissionDirection,
    pub organizer_id: Uuid,
    pub chairman_id: Uuid,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub created_at: NaiveDateTime,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    pub updated_at: NaiveDateTime,
}