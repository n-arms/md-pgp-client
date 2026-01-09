// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn textarea_updated(value: &str) -> String {
    println!("got value {:?}", value.as_bytes());
    format!("updated textarea with value '{value}'")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![textarea_updated])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
