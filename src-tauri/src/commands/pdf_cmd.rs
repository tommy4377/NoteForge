// PDF export via webview print API
// Note: window.print() in JS works on all platforms.
// The Rust-side WebviewWindow::print() only works on macOS.
// We emit an event to the frontend to call window.print() from JS.

use tauri::Emitter;

#[tauri::command]
pub async fn export_pdf(window: tauri::Window) -> Result<(), String> {
    // Emit event to frontend to trigger the browser print dialog
    // The frontend will call window.print() which works on all platforms
    window
        .emit("trigger-print", ())
        .map_err(|e| format!("Failed to emit print event: {}", e))
}
