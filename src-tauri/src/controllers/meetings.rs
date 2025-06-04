use sqlx::types::chrono::NaiveDate;
use uuid::Uuid;

use crate::database::dto::meetings::ReportMeetings;
use crate::database::entity::meetings::Meetings;
use crate::database::repositories::meetings::{
    add_meeting, delete_meeting, get_meeting,
    get_meetings, report_meetings_by_year, update_meeting,
};

#[tauri::command(async)]
pub async fn fetch_meetings_ui() -> Result<Vec<Meetings>, String> {
    match get_meetings().await {
        Ok(meetings) => Ok(meetings),
        Err(reason) => Err(format!("Ошибка при получении заседаний: {}", reason)),
    }
}

#[tauri::command(async)]
pub async fn fetch_meeting_ui(id: String) -> Result<Meetings, String> {
    let id = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID: {}", e))?;
    match get_meeting(id).await {
        Ok(meeting) => Ok(meeting),
        Err(reason) => Err(format!("Ошибка при получении заседания: {}", reason)),
    }
}

#[tauri::command(async)]
pub async fn execute_create_meeting_ui(
    date: String,
    duration: i32,
    commission_id: String,
) -> Result<(), String> {
    let commission_id = Uuid::parse_str(&commission_id)
        .map_err(|e| format!("Неверный формат ID комиссии: {}", e))?;
    let meeting_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|e| format!("Неверный формат даты: {}", e))?;
    if let Err(reason) = add_meeting(meeting_date, duration, commission_id).await {
        Err(format!("Ошибка при создании заседания: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn execute_edit_meeting_ui(
    id: String,
    date: String,
    duration: i32,
    commission_id: String,
) -> Result<(), String> {
    let id = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID заседания: {}", e))?;
    let commission_id = Uuid::parse_str(&commission_id)
        .map_err(|e| format!("Неверный формат ID комиссии: {}", e))?;
    let meeting_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|e| format!("Неверный формат даты: {}", e))?;
    if let Err(reason) = update_meeting(id, meeting_date, duration, commission_id).await {
        Err(format!("Ошибка при обновлении заседания: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn execute_delete_meeting_ui(id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID заседания: {}", e))?;
    if let Err(reason) = delete_meeting(id).await {
        Err(format!("Ошибка при удалении заседания: {}", reason))
    } else {
        Ok(())
    }
}

#[tauri::command(async)]
pub async fn fetch_meetings_by_year_ui(year: i32) -> Result<Vec<ReportMeetings>, String> {
    match report_meetings_by_year(year).await {
        Ok(meeting) => Ok(meeting),
        Err(reason) => Err(format!(
            "Ошибка при получении заседания по годам: {}",
            reason
        )),
    }
}
