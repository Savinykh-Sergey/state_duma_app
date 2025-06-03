use uuid::Uuid;
use crate::database::entity::members::Members;
use crate::database::repositories::chairmen::{delete_chairmen, get_chairmen};

#[tauri::command(async)]
pub async fn fetch_chairmen_ui() -> Result<Vec<Members>, String>{
    match get_chairmen().await {
        Ok(members) => Ok(members),
        Err(reason) => Err(format!("Возникла ошибка при получении членов гос. думы: {}", reason))
    }
}

#[tauri::command(async)]
pub async fn execute_delete_chairman_ui(id: String) -> Result<(), String> {
    let id = Uuid::parse_str(&id).map_err(|e| format!("Неверный формат ID: {}", e))?;

    if let Err(reason) = delete_chairmen(id).await {
        Err(format!("Ошибка при удалении члена госдумы: {}", reason))
    } else {
        Ok(())
    }
}