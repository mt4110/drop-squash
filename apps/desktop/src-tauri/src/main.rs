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
            commands::queue::enqueue_queue_job,
            commands::queue::start_next_queue_job,
            commands::queue::finish_active_queue_job,
            commands::queue::fail_active_queue_job,
            commands::queue::cancel_active_queue_job,
            commands::queue::cancel_queued_job,
            commands::queue::block_queued_jobs,
            commands::queue::clear_completed_queue_jobs,
            commands::trash_original,
            commands::activate_license,
            commands::forget_license
        ])
        .run(tauri::generate_context!())
        .expect("failed to run DropSquash desktop application");
}
