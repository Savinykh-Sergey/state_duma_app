use chrono::NaiveDate;
use uuid::Uuid;

use crate::database::entity::organizers::Organizers;
use crate::database::repositories::organizers::{
    add_organizer, delete_organizer, get_organizer, get_organizers, update_organizer,
};

#[tauri::command(async)]
pub async fn fetch_organizers_ui() -> Result<Vec<Organizers>, String> {
    match get_organizers().await {
        Ok(organizers) => Ok(organizers),
        Err(reason) => Err(format!(
            "Возникла ошибка при получении организаторов: {}",
            reason
        )),
    }
}

#[tauri::command(async)]
pub async fn fetch_organizer_ui(id: String) -> Result<Organizers, String> {
    let id = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID: {}", e))?;

    match get_organizer(id).await {
        Ok(organizer) => Ok(organizer),
        Err(reason) => Err(format!(
            "Возникла ошибка при получении организаторов: {}",
            reason
        )),
    }
}

#[tauri::command(async)]
pub async fn execute_create_organizer_ui(
    full_name: String,
    birthday: String,
    phone: String,
) -> Result<(), String> {
    let birthday_date = NaiveDate::parse_from_str(&birthday, "%Y-%m-%d")
        .map_err(|e| format!("Неверный формат даты: {}", e))?;

    if let Err(reason) = add_organizer(full_name, birthday_date, phone).await {
        Err(format!("Ошибка при обновлении организатора: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn execute_edit_organizer_ui(
    id: String,
    full_name: String,
    birthday: String,
    phone: String,
) -> Result<(), String> {
    let id_uuid = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID: {}", e))?;
    let birthday_date = NaiveDate::parse_from_str(&birthday, "%Y-%m-%d")
        .map_err(|e| format!("Неверный формат даты: {}", e))?;

    if let Err(reason) = update_organizer(id_uuid, full_name, birthday_date, phone).await {
        Err(format!("Ошибка при обновлении организатора: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn execute_delete_organizer_ui(id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID: {}", e))?;

    if let Err(reason) = delete_organizer(id).await {
        Err(format!("Ошибка при удалении организатора: {}", reason))
    } else {
        Ok(())
    }
}
