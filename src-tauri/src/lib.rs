// src/lib.rs
mod command;
mod config;
mod git;
mod wiki;

// 导入命令
use command::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 修复 Linux 下的白屏问题
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    let builder = tauri::Builder::default();

    #[cfg(debug_assertions)]
    let builder = builder.setup(|app| {
        use tauri::Manager;
        if let Some(window) = app.get_webview_window("main") {
            window.open_devtools();
        }
        Ok(())
    });

    builder
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Wiki 命令
            get_wiki_file_structure,
            get_wiki_list,
            create_local_wiki,
            create_remote_wiki,
            delete_wiki,
            create_file,
            create_folder,
            read_file,
            save_file,
            git_sync,
            git_commit_and_sync,
            git_check_status,
            git_set_user_config,
            git_get_user_config,
            setup_remote_repo,
            get_remote_repo_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
