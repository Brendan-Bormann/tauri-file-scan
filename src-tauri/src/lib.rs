use std::sync::Mutex;

mod api;
mod db;
mod file;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_path = "/tmp/file_index.db";
    let conn = db::initialize_db(db_path).expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            api::greet,
            api::index_directory,
            api::search
        ])
        .manage(api::AppState {
            db: Mutex::new(conn),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
