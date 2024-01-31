// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lol_client;

use std::path::Path;

use base64::Engine;
use tauri::Manager;

const DEFAULT_LOL_PATH: &str = "C:\\Riot Games\\League of Legends";

fn main() {
    if !Path::new(DEFAULT_LOL_PATH).exists() {
        return;
    }
    let lock_path = format!("{DEFAULT_LOL_PATH}\\lockfile");

    tauri::Builder::default()
        .setup(|app| {
            let app_handle  = app.app_handle();
            tauri::async_runtime::spawn(lol_client::lol_connection(app_handle, lock_path));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![lol_client::client_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

