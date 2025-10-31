mod git;
mod wiki;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 修复 Linux 下的白屏问题
    #[cfg(target_os = "linux")]
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    let builder = tauri::Builder::default();

    #[cfg(debug_assertions)] // 仅在调试(debug)版本中包含此代码
    let builder = builder.setup(|app| {
        {
            // 自动打开 Devtools
            use tauri::Manager;
            let window = app.get_webview_window("main").unwrap();
            window.open_devtools();
        }
        Ok(())
    });

    builder
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            wiki::command::get_wiki_list,
            wiki::command::get_wiki_file_structure,
            wiki::command::create_local_wiki,
            wiki::command::create_remote_wiki,
            wiki::command::delete_wiki,
            wiki::command::create_file,
            wiki::command::create_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
