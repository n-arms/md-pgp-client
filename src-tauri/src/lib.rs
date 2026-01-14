use crate::rga::{Id, Rga};
use rand::prelude::Rng;
use rand::rng;
use tauri::async_runtime::Mutex;

mod rga;

pub struct AppState {
    rga: Rga,
}

impl AppState {
    pub fn new() -> Self {
        let device: u32 = rng().random();
        let rga = Rga::new(device);
        Self { rga }
    }

    pub fn insert(&mut self, index: usize, char: char) {}
}

#[tauri::command]
fn insert_text(start: isize, end: isize, text: &str, state: tauri::State<'_, Mutex<AppState>>) {}

#[tauri::command]
fn delete_text(start: isize, end: isize, state: tauri::State<'_, Mutex<AppState>>) {
    dbg!(start, end);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::new()))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![insert_text, delete_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
