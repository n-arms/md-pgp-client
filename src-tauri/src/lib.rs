use std::collections::VecDeque;

use crate::rga::{Id, Rga};
use rand::prelude::Rng;
use rand::rng;
use tauri::async_runtime::Mutex;

mod rga;

pub struct AppState {
    rga: Rga,
    outgoing_packets: VecDeque<Packet>,
}

#[derive(Clone, Debug)]
pub enum Packet {
    Insert { parent: Id, char: char },
    Delete { parent: Id },
}

impl AppState {
    pub fn new() -> Self {
        let device: u32 = rng().random();
        let rga = Rga::new(device);
        Self {
            rga,
            outgoing_packets: VecDeque::default(),
        }
    }

    pub fn insert_at_index(&mut self, index: usize, char: char) {
        let id = self.rga.index(index);
        self.rga.insert(char, id);
        self.outgoing_packets
            .push_back(Packet::Insert { parent: id, char });
    }

    pub fn delete_at_index(&mut self, index: usize) {
        let id = self.rga.index(index);
        self.rga.delete(id);
        self.outgoing_packets
            .push_back(Packet::Delete { parent: id });
    }
}

#[tauri::command]
async fn insert_text(
    start: isize,
    end: isize,
    text: &str,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), ()> {
    println!("insert at {start}..{end} with {text:?}");
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
    println!("delete at {start}..{end}");
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
async fn get_text(state: tauri::State<'_, Mutex<AppState>>) -> Result<String, ()> {
    Ok(state.lock().await.rga.to_list())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::new()))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![insert_text, delete_text, get_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
