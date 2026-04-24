mod commands {
    pub mod file_cmds;
    pub mod pdf_cmd;
    pub mod platform_cmds;
    pub mod recent_cmds;
    pub mod watcher_cmd;
    pub mod window_cmds;
}

use commands::watcher_cmd::FileWatcherState;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_process::init())
        .manage(Mutex::new(FileWatcherState {
            watcher: None,
            watched_paths: Default::default(),
        }))
        .invoke_handler(tauri::generate_handler![
            commands::file_cmds::open_file,
            commands::file_cmds::save_file,
            commands::file_cmds::get_file_language,
            commands::pdf_cmd::export_pdf,
            commands::platform_cmds::reveal_in_explorer,
            commands::recent_cmds::get_recent_files,
            commands::recent_cmds::add_recent_file,
            commands::watcher_cmd::start_file_watcher,
            commands::watcher_cmd::stop_file_watcher,
            commands::window_cmds::set_window_title,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
