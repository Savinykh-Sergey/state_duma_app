use uuid::Uuid;
use crate::database::entity::commission_members::CommissionMembers;
use crate::database::repositories::commission_members::{add_commission_member, edit_commission_member, get_commission_member};

#[tauri::command(async)]
pub async fn execute_create_commission_members_ui(member_id: String, commission_id: String) -> Result<(), String> {
    let member_id = Uuid::parse_str(&member_id).map_err(|e| format!("Неверный формат ID: {}", e))?;
    let commission_id = Uuid::parse_str(&commission_id).map_err(|e| format!("Неверный формат ID: {}", e))?;

    if let Err(reason) = add_commission_member(member_id, commission_id).await {
        Err(format!("Возникла ошибка при получении комиссий: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn execute_edit_commission_members_ui(member_id: String, commission_id: String) -> Result<(), String> {
    let member_id = Uuid::parse_str(&member_id).map_err(|e| format!("Неверный формат ID: {}", e))?;
    let commission_id = Uuid::parse_str(&commission_id).map_err(|e| format!("Неверный формат ID: {}", e))?;

    if let Err(reason) = edit_commission_member(member_id, commission_id).await {
        Err(format!("Возникла ошибка при получении комиссий: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn fetch_commission_member_ui(member_id: String) -> Result<CommissionMembers, String> {
    let member_id = Uuid::parse_str(&member_id).map_err(|e| format!("Неверный формат ID: {}", e))?;

    match get_commission_member(member_id).await {
        Ok(commission_member) => {
            println!("commission_member: {:?}", commission_member);
            Ok(commission_member)
        },
        Err(reason) => Err(format!("Возникла ошибка при получении комиссий: {}", reason)),
    }
}
