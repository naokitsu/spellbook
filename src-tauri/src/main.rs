// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;
use std::sync::Mutex;
use base64::{Engine, engine::general_purpose::STANDARD};
use tauri::Manager;
use tokio::time::{sleep, Duration};
use lazy_static::lazy_static;
use windows_sys::Win32::System::Threading::{INFINITE, PROCESS_SYNCHRONIZE};


const DEFAULT_LOL_PATH: &str = "C:\\Riot Games\\League of Legends";

#[derive(Debug)]
struct Connection {
    process_id: u32,
    port: u16,
    token: String,
}
type AppState = Mutex<Option<Connection>>;

#[derive(Clone, serde::Serialize)]
enum LoLClientState {
    Offline,
    Online,
}

lazy_static!{
    static ref APP_STATE: AppState = Mutex::new(None);
}

#[tauri::command]
fn client_state() -> LoLClientState {
    match *APP_STATE.lock().unwrap() {
        Some(_) => LoLClientState::Online,
        None => LoLClientState::Offline,
    }
}


fn main() {
    if !Path::new(DEFAULT_LOL_PATH).exists() {
        return;
    }

    tauri::Builder::default()
        .setup(|app| {
            let app_handle  = app.app_handle();

            let lock_path = format!("{DEFAULT_LOL_PATH}\\lockfile");
            tauri::async_runtime::spawn(async move {
                let app_handle = app_handle;
                'file_doesnt_exist: loop {
                    sleep(Duration::from_millis(1_000)).await;
                    while !Path::new(&lock_path).exists() {
                        sleep(Duration::from_millis(1_000)).await;
                    }
                    let process_id = if let Ok(line) = fs::read_to_string(&lock_path)  {
                        let mut words = line.split(':').skip(1);
                        let process_id = words.next().unwrap().parse().unwrap();
                        let port = words.next().unwrap().parse().unwrap();
                        let password = words.next().unwrap();
                        let token = STANDARD.encode(format!("riot:{password}").as_str());
                        *APP_STATE.lock().unwrap() = Some(Connection{ process_id, port, token });
                        process_id
                    } else {
                        continue 'file_doesnt_exist;
                    };
                    app_handle.emit_all("LoLClientEvent", LoLClientState::Online);
                    unsafe {
                        let handle = windows_sys::Win32::System::Threading::OpenProcess(PROCESS_SYNCHRONIZE, 0, process_id);
                        let wait_result = windows_sys::Win32::System::Threading::WaitForSingleObject(handle, INFINITE);
                    };
                    *APP_STATE.lock().unwrap() = None;
                    app_handle.emit_all("LoLClientEvent", LoLClientState::Offline);

                }
            });
            Ok(())
        })

        .invoke_handler(tauri::generate_handler![client_state])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

