use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

use crate::file::{index_files, search_files};

pub struct AppState {
    pub db: Mutex<Connection>,
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn index_directory(state: State<AppState>, dir: String) -> Result<(), String> {
    let conn = state.db.lock().unwrap();
    index_files(&conn, &dir).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn search(state: State<AppState>, query: String) -> Result<Vec<String>, String> {
    let conn = state.db.lock().unwrap();
    search_files(&conn, &query).map_err(|e| e.to_string())
}
