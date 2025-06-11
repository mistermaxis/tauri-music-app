extern crate dirs;
use crate::project_mods::{lofty::load_lofty_files, song::Song};

mod project_mods;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn get_songs() -> Vec<project_mods::song::Song> {
    let song_list = load_lofty_files();
    match song_list {
      Some(list) => list,
      None => Vec::<Song>::new()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_songs])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
