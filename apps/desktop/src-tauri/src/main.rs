mod commands;
mod state;

fn main() {
    tauri::Builder::default()
        .manage(state::AppState::default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::load_state,
            commands::save_config,
            commands::convert,
            commands::cancel_conversion,
            commands::trash_original,
            commands::activate_license,
            commands::forget_license
        ])
        .run(tauri::generate_context!())
        .expect("failed to run DropSquash desktop application");
}
