pub mod auth;
pub mod cache;
pub mod commands;
pub mod config;
pub mod discord;
pub mod friends;
pub mod game;
pub mod manifest;
pub mod launcher;

use commands::*;

use std::sync::Arc;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Arc::new(DownloadControl::default()))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            initialize_discord_rpc,
            disconnect_discord_rpc,
            set_idle_activity,
            set_playing_activity,
            loginu,
            loginc,
            saveu,
            fetchsu,
            fetchcu,
            fetchcc,
            get_progress,
            cancel_download,
            resume_download,
            start_download,
            download_complete,
            uninstall_complete,
            start_verify,
            start_uninstall,
            fetch_installed_object_by_artifact_id,
            update_installed_object_by_artifact_id,
            push_installed_object,
            get_drives,
            launch_game,
            get_is_playing,
            fetch_friends_list,
            fetch_incoming_friends_list,
            fetch_outgoing_friends_list,
            fetch_blocklist,
            fetch_display_name_by_account_id,
            accept_friend_request,
            decline_friend_request,
            get_game_information
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
