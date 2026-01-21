use crate::rga::{DeviceId, Id, Rga};
use rand::prelude::Rng;
use rand::rng;
use serde_json::Value;
use tauri::async_runtime::Mutex;
use tauri::{AppHandle, Runtime, Wry};
use tauri_plugin_store::StoreExt;

mod rga;

pub struct AppState {
    file: Option<OpenFile>,
    server_addr: Option<String>,
    device: DeviceId,
}

pub struct OpenFile {
    rga: Rga,
}

#[derive(Clone, Debug)]
pub enum Packet {
    Insert { parent: Id, char: char },
    Delete { parent: Id },
}

impl AppState {
    pub fn new() -> Self {
        Self {
            file: None,
            server_addr: None,
            device: DeviceId(rng().random()),
        }
    }

    pub fn insert_at_index(&mut self, index: usize, char: char) {
        if let Some(file) = &mut self.file {
            let id = file.rga.index(index);
            file.rga.insert(char, id);
            self.push_packet(Packet::Insert { parent: id, char });
        }
    }

    pub fn delete_at_index(&mut self, index: usize) {
        if let Some(file) = &mut self.file {
            let id = file.rga.index(index);
            file.rga.delete(id);
            self.push_packet(Packet::Delete { parent: id });
        }
    }

    pub fn push_packet(&mut self, packet: Packet) {}
}

#[tauri::command]
async fn insert_text(
    start: isize,
    end: isize,
    text: &str,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), ()> {
    let mut app_state = state.lock().await;
    for i in ((start + 1)..=end).rev() {
        app_state.delete_at_index(i as usize);
    }
    for (i, char) in text.char_indices() {
        app_state.insert_at_index(start as usize + i, char);
    }
    Ok(())
}

#[tauri::command]
async fn delete_text(
    start: isize,
    end: isize,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), ()> {
    let mut app_state = state.lock().await;
    if start == end {
        app_state.delete_at_index(start as usize);
    } else {
        for i in ((start + 1)..=end).rev() {
            app_state.delete_at_index(i as usize);
        }
    }
    Ok(())
}

#[tauri::command]
async fn get_text(state: tauri::State<'_, Mutex<AppState>>) -> Result<String, String> {
    if let Some(file) = state.lock().await.file.as_ref() {
        Ok(file.rga.to_list())
    } else {
        Err("No open file".into())
    }
}

#[tauri::command]
async fn load_store(
    app: AppHandle,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let path = std::path::PathBuf::from("settings.json");

    let store = app.store(&path).map_err(|e| e.to_string())?;

    let has_setup = store
        .get("hasSetup")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if has_setup {
        let server_address = store
            .get("serverAddress")
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap();
        state.lock().await.server_addr = Some(server_address);
    }

    Ok(())
}

#[tauri::command]
async fn open_new_file(state: tauri::State<'_, Mutex<AppState>>) -> Result<(), ()> {
    let mut state = state.lock().await;
    state.file = Some(OpenFile {
        rga: Rga::new(state.device),
    });
    Ok(())
}

#[tauri::command]
async fn close_file(state: tauri::State<'_, Mutex<AppState>>) -> Result<(), ()> {
    state.lock().await.file = None;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::new()))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            insert_text,
            delete_text,
            get_text,
            load_store,
            open_new_file,
            close_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
