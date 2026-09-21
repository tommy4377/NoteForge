// File operations: open, save, detect encoding

use encoding_rs::Encoding;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Clone)]
pub struct FileInfo {
    pub content: String,
    pub encoding: String,
    pub line_ending: String, // "LF" or "CRLF"
    pub language: String,
}

/// Detect the language from file extension
fn detect_language(path: &str) -> String {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "md" | "markdown" => "markdown",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "css" | "scss" => "css",
        "html" | "htm" => "html",
        "js" | "mjs" | "cjs" => "javascript",
        "ts" => "javascript",
        "py" => "python",
        "rs" => "rust",
        "txt" | "log" => "text",
        "csv" => "csv",
        "xml" => "xml",
        _ => "text",
    }
    .to_string()
}

/// Detect encoding of raw bytes using chardetng
fn detect_encoding(bytes: &[u8]) -> &'static Encoding {
    let mut detector = chardetng::EncodingDetector::new(chardetng::Iso2022JpDetection::Allow);
    detector.feed(bytes, true);
    detector.guess(None, chardetng::Utf8Detection::Allow)
}

#[tauri::command]
pub fn open_file(path: String) -> Result<FileInfo, String> {
    let bytes = fs::read(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    let encoding = detect_encoding(&bytes);
    let (content, encoding_used, _had_errors) = encoding.decode(&bytes);

    let encoding_name = encoding_used.name().to_string();

    // Detect line ending
    let line_ending = if content.contains('\r') {
        "CRLF".to_string()
    } else {
        "LF".to_string()
    };

    let language = detect_language(&path);

    Ok(FileInfo {
        content: content.into_owned(),
        encoding: encoding_name,
        line_ending,
        language,
    })
}

#[tauri::command]
pub fn save_file(
    path: String,
    content: String,
    encoding: String,
    line_ending: String,
) -> Result<(), String> {
    // Convert line endings
    let normalized_content = match line_ending.as_str() {
        "CRLF" => content.replace('\n', "\r\n"),
        _ => content,
    };

    let enc = Encoding::for_label(encoding.as_bytes()).unwrap_or(encoding_rs::UTF_8);

    let (encoded_bytes, _, _had_errors) = enc.encode(&normalized_content);

    // Ensure parent directory exists
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    fs::write(&path, encoded_bytes.as_ref()).map_err(|e| format!("Failed to write file: {}", e))
}

#[tauri::command]
pub fn get_file_language(path: String) -> String {
    detect_language(&path)
}
