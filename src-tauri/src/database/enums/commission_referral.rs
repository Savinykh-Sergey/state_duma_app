use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type, Clone, Debug)]
#[sqlx(type_name = "commission_direction")]
pub enum CommissionDirection {
    Social,
    Ecological,
}