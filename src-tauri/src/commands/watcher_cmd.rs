// File watcher — monitors open files for external changes

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter};

pub struct FileWatcherState {
    pub watcher: Option<RecommendedWatcher>,
    pub watched_paths: HashMap<String, PathBuf>,
}

#[tauri::command]
pub fn start_file_watcher(
    app_handle: AppHandle,
    path: String,
    state: tauri::State<'_, Mutex<FileWatcherState>>,
) -> Result<(), String> {
    let mut watcher_state = state.lock().map_err(|e| e.to_string())?;

    // If we already have a watcher, stop it
    if watcher_state.watcher.is_some() {
        let _ = watcher_state.watcher.take();
    }

    let watch_path = PathBuf::from(&path);
    let app_handle_clone = app_handle.clone();
    let path_clone = path.clone();

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| match res {
            Ok(event) => match event.kind {
                EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_) => {
                    let _ = app_handle_clone.emit("file-changed", &path_clone);
                }
                _ => {}
            },
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
            }
        },
        Config::default(),
    )
    .map_err(|e| format!("Failed to create watcher: {}", e))?;

    watcher
        .watch(&watch_path, RecursiveMode::NonRecursive)
        .map_err(|e| format!("Failed to start watching: {}", e))?;

    watcher_state.watched_paths.insert(path.clone(), watch_path);
    watcher_state.watcher = Some(watcher);

    Ok(())
}

#[tauri::command]
pub fn stop_file_watcher(state: tauri::State<'_, Mutex<FileWatcherState>>) -> Result<(), String> {
    let mut watcher_state = state.lock().map_err(|e| e.to_string())?;
    watcher_state.watcher = None;
    watcher_state.watched_paths.clear();
    Ok(())
}
