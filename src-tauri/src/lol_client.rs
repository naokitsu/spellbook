use std::fs;
use std::path::Path;
use std::sync::Mutex;
use base64::{Engine, engine::general_purpose::STANDARD};
use tauri::{AppHandle, Manager};
use tokio::time::{sleep, Duration};
use lazy_static::lazy_static;
use windows_sys::Win32::System::Threading::{INFINITE, PROCESS_SYNCHRONIZE};


#[derive(Clone, serde::Serialize)]
pub(super) enum LoLClientState {
    Offline,
    Online,
}

#[derive(Debug)]
pub(super) struct Connection {
    process_id: u32,
    port: u16,
    token: String,
}

type AppState = Mutex<Option<Connection>>;

lazy_static!{
    static ref APP_STATE: AppState = Mutex::new(None);
}

#[tauri::command]
pub(super) fn client_state() -> LoLClientState {
    match *APP_STATE.lock().unwrap() {
        Some(_) => LoLClientState::Online,
        None => LoLClientState::Offline,
    }
}

pub(super) async fn lol_connection(app_handle: AppHandle, lock_path: String) {
    'file_doesnt_exist: loop {
        sleep(Duration::from_millis(1000)).await;
        while !Path::new(&lock_path).exists() {
            sleep(Duration::from_millis(1000)).await;
        }
        let process_id = if let Ok(line) = fs::read_to_string(&lock_path) {
            let mut words = line.split(':').skip(1);
            let process_id = words.next().unwrap().parse().unwrap();
            let port = words.next().unwrap().parse().unwrap();
            let password = words.next().unwrap();
            let token = STANDARD.encode(format!("riot:{password}").as_str());
            *APP_STATE.lock().unwrap() = Some(Connection { process_id, port, token });
            process_id
        } else {
            continue 'file_doesnt_exist;
        };
        if let Err(e) = app_handle.emit_all("LoLClientEvent", LoLClientState::Online) {
            eprintln!("Failed to send online event: {e}")
        };
        unsafe {
            let handle = windows_sys::Win32::System::Threading::OpenProcess(PROCESS_SYNCHRONIZE, 0, process_id);
            let _ = windows_sys::Win32::System::Threading::WaitForSingleObject(handle, INFINITE);
        };
        *APP_STATE.lock().unwrap() = None;
        if let Err(e) = app_handle.emit_all("LoLClientEvent", LoLClientState::Offline) {
            eprintln!("Failed to send offline event: {e}")
        };

    }

}