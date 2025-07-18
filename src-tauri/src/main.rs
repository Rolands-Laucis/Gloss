// src/main.rs - Tauri Application
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![allow(warnings)] //turn off warnings

// mod wordnet_v2;
// use wordnet_v2::{initialize_wordnet_index, search_words};
mod wordnet_v3;
use wordnet_v3::{init_wordnet, search_wordnet};

fn main() {
    // Initialize once at startup
    init_wordnet("resources/en_wordnet_lmf_2024.json", "en")
        .expect("Failed to load English WordNet");
    init_wordnet("resources/lv_wordnet_lmf_2025.json", "lv")
        .expect("Failed to load Latvian WordNet");

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![search_wordnet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
