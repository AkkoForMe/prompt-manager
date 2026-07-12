// 应用入口

mod commands;
mod db;
mod models;

use db::DbState;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // 获取应用数据目录
            let app_data_dir = app.path().app_data_dir().expect("无法获取应用数据目录");

            // 确保目录存在
            std::fs::create_dir_all(&app_data_dir).expect("无法创建应用数据目录");

            // 数据库文件路径
            let db_path = app_data_dir.join("prompt-manager.db");

            // 打开数据库连接
            let conn = rusqlite::Connection::open(&db_path).expect("无法打开数据库");

            // 启用外键约束，并开启 WAL 模式提升性能
            conn.execute_batch(
                "PRAGMA foreign_keys=ON;
                 PRAGMA journal_mode=WAL;",
            )
            .expect("设置数据库 PRAGMA 失败");

            // 初始化表结构
            db::init_db(&conn).expect("初始化数据库失败");

            // 将数据库连接存入状态管理
            app.manage(DbState(Mutex::new(conn)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_prompts,
            commands::create_prompt,
            commands::update_prompt,
            commands::delete_prompt,
            commands::set_prompt_pinned,
            commands::record_prompt_use,
            commands::get_all_tags,
            commands::get_categories,
            commands::create_category,
            commands::update_category,
            commands::delete_category,
            commands::get_uncategorized_count,
            commands::export_to_json,
            commands::export_to_markdown,
            commands::preview_import_json,
            commands::import_from_json,
        ])
        .run(tauri::generate_context!())
        .expect("运行应用时出错");
}
