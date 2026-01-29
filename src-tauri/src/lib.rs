use std::path::PathBuf;

use crate::e2ee::{build_signed_message, read_skey_file};
use crate::rga::{Id, Rga};
use pgp::composed::SignedSecretKey;
use pgp::crypto::hash::HashAlgorithm;
use pgp::ser::Serialize;
use rand::{prelude::Rng, thread_rng, RngCore};
use tauri::async_runtime::Mutex;
use tauri::{AppHandle, Runtime, Wry};
use tauri_plugin_store::StoreExt;

mod config;
mod e2ee;
mod rga;

pub struct AppState {
    rga: Rga,
    server_address: Option<PathBuf>,
    key: Option<SignedSecretKey>,
}

#[derive(Clone, Debug)]
pub enum Packet {
    Insert { parent: Id, char: char },
    Delete { parent: Id },
}

impl AppState {
    pub fn new() -> Self {
        let device: u32 = thread_rng().next_u32();
        let rga = Rga::new(device);
        Self {
            rga,
            server_address: None,
            key: None,
        }
    }

    pub fn insert_at_index(&mut self, index: usize, char: char) {
        let id = self.rga.index(index);
        self.rga.insert(char, id);
        self.push_packet(Packet::Insert { parent: id, char });
    }

    pub fn delete_at_index(&mut self, index: usize) {
        let id = self.rga.index(index);
        self.rga.delete(id);
        self.push_packet(Packet::Delete { parent: id });
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

#[tauri::command]
async fn create_account(state: tauri::State<'_, Mutex<AppState>>) -> Result<(), String> {
    let mut url = state.lock().await.server_address.as_ref().unwrap().clone();
    let skey = state.lock().await.key.as_ref().unwrap().clone();
    url.push("create_account");
    let pkey = skey.signed_public_key();
    let packet_contents = build_signed_message(
        &skey,
        &pkey.to_bytes().unwrap(),
        &mut thread_rng(),
        HashAlgorithm::Sha256,
    )
    .map_err(|err| err.to_string())?;
    let client = reqwest::Client::new();
    client
        .post(url.to_str().unwrap())
        .body(packet_contents)
        .send()
        .await
        .map_err(|err| {
            format!(
                "Building client request with url {}:\n{err}",
                url.to_str().unwrap()
            )
        })?;
    Ok(())
}

#[tauri::command]
async fn create_document(
    name: &str,
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    let mut url = state.lock().await.server_address.as_ref().unwrap().clone();
    let skey = state.lock().await.key.as_ref().unwrap().clone();
    url.push("create_document");
    let packet_contents = build_signed_message(
        &skey,
        name.as_bytes(),
        &mut thread_rng(),
        HashAlgorithm::Sha256,
    )
    .map_err(|err| err.to_string())?;
    let client = reqwest::Client::new();
    client
        .post(url.to_str().unwrap())
        .body(packet_contents)
        .send()
        .await
        .map_err(|err| {
            format!(
                "Building client request with url {}:\n{err}",
                url.to_str().unwrap()
            )
        })?;
    Ok(())
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
        state.lock().await.server_address = Some(server_address.into());
        let key_path = store
            .get("keyPath")
            .and_then(|v| v.as_str().map(ToOwned::to_owned))
            .unwrap_or(String::new());
        let skey = read_skey_file(key_path).map_err(|err| err.to_string())?;
        state.lock().await.key = Some(skey);
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState::new()))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            insert_text,
            delete_text,
            get_text,
            create_account,
            load_store,
            create_document
        ])
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
