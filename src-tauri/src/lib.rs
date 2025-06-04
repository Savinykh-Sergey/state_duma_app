mod controllers;
mod database;
mod types;

use crate::controllers::chairmen::{execute_delete_chairman_ui, fetch_chairmen_ui};
use crate::controllers::commission_members::{
    execute_create_commission_members_ui, execute_edit_commission_members_ui,
    fetch_commission_member_ui,
};
use crate::controllers::commissions::{
    execute_create_commission_ui, execute_delete_commission_ui, execute_edit_commission_ui,
    fetch_commission_ui, fetch_commissions_ui, fetch_members_by_direction_report_ui,
};
use crate::controllers::meetings::{
    execute_create_meeting_ui, execute_delete_meeting_ui, execute_edit_meeting_ui,
    fetch_meeting_ui, fetch_meetings_by_year_ui, fetch_meetings_ui,
};
use crate::controllers::members::{
    execute_create_member_ui, execute_delete_member_ui, execute_edit_member_ui, fetch_member_ui,
    fetch_members_ui, fetch_report_experienced_member_ui, fetch_report_retired_member_ui,
};
use crate::controllers::organizers::{
    execute_create_organizer_ui, execute_delete_organizer_ui, execute_edit_organizer_ui,
    fetch_organizer_ui, fetch_organizers_ui,
};
use crate::database::init::init_pools;
use dotenv::dotenv;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    dotenv().ok();
    init_pools().await;

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_members_ui,
            fetch_member_ui,
            execute_edit_member_ui,
            execute_create_member_ui,
            execute_delete_member_ui,
            fetch_organizers_ui,
            fetch_organizer_ui,
            execute_create_organizer_ui,
            execute_edit_organizer_ui,
            execute_delete_organizer_ui,
            fetch_chairmen_ui,
            execute_delete_chairman_ui,
            fetch_commissions_ui,
            fetch_commission_ui,
            execute_create_commission_ui,
            execute_edit_commission_ui,
            execute_delete_commission_ui,
            fetch_meetings_ui,
            fetch_meeting_ui,
            execute_create_meeting_ui,
            execute_edit_meeting_ui,
            execute_delete_meeting_ui,
            fetch_commission_member_ui,
            execute_create_commission_members_ui,
            execute_edit_commission_members_ui,
            fetch_report_experienced_member_ui,
            fetch_report_retired_member_ui,
            fetch_members_by_direction_report_ui,
            fetch_meetings_by_year_ui
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
