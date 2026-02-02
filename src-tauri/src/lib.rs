// 模块声明
mod models;
mod game_engine;
mod ai;
mod commands;
mod game_session;
mod error_logger;

// 导入命令
use commands::{
    start_new_game,
    get_legal_moves,
    make_player_move,
    make_ai_move,
    undo_move,
    restart_game,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            start_new_game,
            get_legal_moves,
            make_player_move,
            make_ai_move,
            undo_move,
            restart_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
