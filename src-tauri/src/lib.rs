// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn insert_text(start: isize, end: isize, text: &str) {
    dbg!(start, end, text);
}

#[tauri::command]
fn delete_text(start: isize, end: isize) {
    dbg!(start, end);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![insert_text, delete_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
