// Recent files management — persisted via tauri-plugin-store

use serde::{Deserialize, Serialize};
use tauri_plugin_store::StoreExt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecentFile {
    pub path: String,
    pub name: String,
    pub last_opened: u64, // unix timestamp
}

#[tauri::command]
pub fn get_recent_files(app_handle: tauri::AppHandle) -> Result<Vec<RecentFile>, String> {
    let store = app_handle
        .store("recent_files.json")
        .map_err(|e| format!("Failed to open store: {}", e))?;

    let recent: Vec<RecentFile> = store
        .get("recent_files")
        .and_then(|v: serde_json::Value| serde_json::from_value::<Vec<RecentFile>>(v).ok())
        .unwrap_or_default();

    Ok(recent)
}

#[tauri::command]
pub fn add_recent_file(
    path: String,
    name: String,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let store = app_handle
        .store("recent_files.json")
        .map_err(|e| format!("Failed to open store: {}", e))?;

    let mut recent: Vec<RecentFile> = store
        .get("recent_files")
        .and_then(|v: serde_json::Value| serde_json::from_value::<Vec<RecentFile>>(v).ok())
        .unwrap_or_default();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    // Remove if already exists
    recent.retain(|f| f.path != path);

    // Add to front
    recent.insert(
        0,
        RecentFile {
            path: path.clone(),
            name: name.clone(),
            last_opened: timestamp,
        },
    );

    // Keep only last 20
    recent.truncate(20);

    store.set("recent_files", serde_json::to_value(&recent).unwrap());
    store
        .save()
        .map_err(|e| format!("Failed to save store: {}", e))?;

    Ok(())
}
