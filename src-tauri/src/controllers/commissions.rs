use uuid::Uuid;

use crate::database::dto::commissions::ReportCommission;
use crate::database::entity::commissions::Commissions;
use crate::database::enums::commission_referral::CommissionDirection;
use crate::database::repositories::commissions::{
    add_commission, delete_commission, get_commission, get_commission_members_by_direction,
    get_commissions, update_commission,
};

#[tauri::command(async)]
pub async fn fetch_commissions_ui() -> Result<Vec<Commissions>, String> {
    match get_commissions().await {
        Ok(commissions) => Ok(commissions),
        Err(reason) => Err(format!(
            "Возникла ошибка при получении комиссий: {}",
            reason
        )),
    }
}

#[tauri::command(async)]
pub async fn fetch_commission_ui(id: String) -> Result<Commissions, String> {
    let id = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID: {}", e))?;

    match get_commission(id).await {
        Ok(commission) => Ok(commission),
        Err(reason) => Err(format!(
            "Возникла ошибка при получении комиссии: {}",
            reason
        )),
    }
}

#[tauri::command(async)]
pub async fn execute_create_commission_ui(
    direction: String,
    chairman_id: String,
    organizer_id: String,
) -> Result<(), String> {
    let chairman_id = Uuid::parse_str(&chairman_id)
        .map_err(|e| format!("Неверный формат ID председателя: {}", e))?;
    let organizer_id = Uuid::parse_str(&organizer_id)
        .map_err(|e| format!("Неверный формат ID организатора: {}", e))?;

    let direction_enum = match direction.as_str() {
        "Social" => CommissionDirection::Social,
        "Ecological" => CommissionDirection::Ecological,
        _ => return Err(format!("Недопустимое значение направления: {}", direction)),
    };

    if let Err(reason) = add_commission(direction_enum, chairman_id, organizer_id).await {
        Err(format!("Ошибка при создании комиссии: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn execute_edit_commission_ui(
    id: String,
    direction: String,
    chairman_id: String,
    organizer_id: String,
) -> Result<(), String> {
    let id = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID: {}", e))?;
    let chairman_id = Uuid::parse_str(&chairman_id)
        .map_err(|e| format!("Неверный формат ID председателя: {}", e))?;
    let organizer_id = Uuid::parse_str(&organizer_id)
        .map_err(|e| format!("Неверный формат ID организатора: {}", e))?;

    let direction_enum = match direction.as_str() {
        "Social" => CommissionDirection::Social,
        "Ecological" => CommissionDirection::Ecological,
        _ => return Err(format!("Недопустимое значение направления: {}", direction)),
    };

    if let Err(reason) = update_commission(id, direction_enum, chairman_id, organizer_id).await {
        Err(format!("Ошибка при обновлении комиссии: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn execute_delete_commission_ui(id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID: {}", e))?;

    if let Err(reason) = delete_commission(id).await {
        Err(format!("Ошибка при удалении комиссии: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn fetch_members_by_direction_report_ui(
    direction: String,
) -> Result<Vec<ReportCommission>, String> {
    println!("{:?}", direction);

    let direction_enum = match direction.as_str() {
        "social" => CommissionDirection::Social,
        "ecological" => CommissionDirection::Ecological,
        _ => return Err(format!("Недопустимое значение направления: {}", direction)),
    };

    match get_commission_members_by_direction(direction_enum).await {
        Ok(members) => Ok(members),
        Err(reason) => Err(format!(
            "Ошибка при получении членов думы по направлению: {}",
            reason
        )),
    }
}
