use chrono::NaiveDate;
use uuid::Uuid;
use crate::database::dto::members::{ExperienceReport, RetiredReport};
use crate::database::entity::members::Members;
use crate::database::repositories::members::{add_member, delete_member, get_member, get_members, get_retired_members, report_experienced_members, update_member};

#[tauri::command(async)]
pub async fn fetch_members_ui() -> Result<Vec<Members>, String>{
    match get_members().await {
        Ok(members) => Ok(members),
        Err(reason) => Err(format!("Возникла ошибка при получении членов гос. думы: {}", reason))
    }
}

#[tauri::command(async)]
pub async fn fetch_member_ui(query: String) -> Result<Members, String> {
    match get_member(query).await {
        Ok(member) => Ok(member),
        Err(reason) => Err(format!("Возникла ошибка при получении члена гос. думы: {}", reason))
    }
}

#[tauri::command(async)]
pub async fn execute_create_member_ui(full_name: String, birthday: String, experience: i32, phone: String, is_chairman: bool) -> Result<(), String> {
    let birthday_date = NaiveDate::parse_from_str(&birthday, "%Y-%m-%d").map_err(|e| format!("Неверный формат даты: {}", e))?;

    if let Err(reason) = add_member(full_name, birthday_date, experience, phone, is_chairman).await {
        Err(format!("Ошибка при обновлении члена госдумы: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn execute_edit_member_ui(id: String, full_name: String, birthday: String, experience: i32, phone: String, is_chairman: bool) -> Result<(), String> {
    let id_uuid = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID: {}", e))?;
    let birthday_date = NaiveDate::parse_from_str(&birthday, "%Y-%m-%d").map_err(|e| format!("Неверный формат даты: {}", e))?;

    if let Err(reason) = update_member(id_uuid, full_name, birthday_date, experience, phone, is_chairman).await {
        Err(format!("Ошибка при обновлении члена госдумы: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn execute_delete_member_ui(id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID: {}", e))?;

    if let Err(reason) = delete_member(id).await {
        Err(format!("Ошибка при удалении члена госдумы: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn fetch_report_experienced_member_ui() -> Result<Vec<ExperienceReport>, String>{
    match report_experienced_members().await {
        Ok(members) => Ok(members),
        Err(reason) => Err(format!("Возникла ошибка при получении членов гос. думы: {}", reason))
    }
}

#[tauri::command(async)]
pub async fn fetch_report_retired_member_ui() -> Result<Vec<RetiredReport>, String>{
    match get_retired_members().await {
        Ok(members) => Ok(members),
        Err(reason) => Err(format!("Возникла ошибка при получении членов гос. думы: {}", reason))
    }
}