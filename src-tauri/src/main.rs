// src/main.rs - Tauri Application
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(warnings)] //turn off warnings

mod wordnet_v2;
use wordnet_v2::{initialize_wordnet_index, search_words};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            search_words
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    wordnet_v2::initialize_wordnet_index("resources/wordnet.json")
        .map(|_| "WordNet initialized on startup".to_string())
        .map_err(|e| format!("Startup initialization failed: {}", e));
}

// Alternative: Initialize WordNet on app startup
// #[tauri::command]
// fn initialize_on_startup() -> Result<String, String> {
//     wordnet_v2::initialize_wordnet_index("resources/wordnet.json")
//         .map(|_| "WordNet initialized on startup".to_string())
//         .map_err(|e| format!("Startup initialization failed: {}", e))
// }

// If you want to auto-initialize on startup, add this to your Builder:
// .setup(|app| {
//     let _ = initialize_on_startup();
//     Ok(())
// })