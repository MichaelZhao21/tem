// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod temsync;

#[tauri::command]
fn load_tags() -> Vec<String> {
    let out = vec!["first".to_string(), "second".to_string(), "third".to_string()];
    return out;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_tags])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // TEMP to get rid of the annoying unused code warnings
    temsync::main();
}
