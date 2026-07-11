mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::load_state,
            commands::convert
        ])
        .run(tauri::generate_context!())
        .expect("failed to run DropSquash desktop application");
}
