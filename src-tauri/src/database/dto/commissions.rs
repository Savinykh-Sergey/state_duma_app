use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ReportCommission {
    pub full_name: String,
    pub is_chairman: bool,
}