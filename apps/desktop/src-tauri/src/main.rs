mod commands;
mod eject_helper;
mod opened_files;
mod single_instance;
mod state;

use tauri::Manager;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(result) = eject_helper::run_from_args(&args) {
        result.expect("failed to eject installer volume");
        return;
    }
    let single_instance =
        single_instance::prepare().expect("failed to prepare DropSquash single-instance guard");
    let Some(single_instance) = single_instance else {
        return;
    };
    let mut builder = tauri::Builder::default();
    if !single_instance::qa_parallel_instance_requested() {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            single_instance::focus_existing(app);
        }));
    }
    let app = builder
        .setup(move |app| {
            single_instance.attach(app.handle());
            Ok(())
        })
        .manage(state::AppState::default())
        .manage(commands::secure_share::SecureShareRecordingState::default())
        .plugin(opened_files::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .on_window_event(|window, event| {
            commands::window_events::record_window_event(window, event)
        })
        .invoke_handler(tauri::generate_handler![
            commands::load_state,
            commands::load_install_location,
            commands::copy_to_applications,
            commands::eject_installer_volume,
            commands::open_installed_application,
            commands::install::reveal_finder_item,
            commands::quit_current_app,
            commands::quit_after_installer_volume_eject,
            commands::save_config,
            commands::convert,
            commands::cancel_conversion,
            commands::record_manual_qa_event,
            commands::qa::secure_share::manual_qa_secure_share_observe_window,
            commands::qa::secure_share::alpha::secure_share_alpha_windows,
            commands::qa::secure_share::alpha::secure_share_alpha_capture,
            commands::qa::secure_share::alpha::recording::secure_share_alpha_record,
            commands::secure_share::targets::secure_share_recording_targets,
            commands::secure_share::permission::open_accessibility_settings,
            commands::secure_share::permission::open_screen_capture_settings,
            commands::secure_share::recording::secure_share_recording_start,
            commands::secure_share::recording::secure_share_recording_stop,
            commands::queue::enqueue_queue_job,
            commands::queue::enqueue_files,
            commands::queue::start_next_queue_job,
            commands::queue::finish_active_queue_job,
            commands::queue::fail_active_queue_job,
            commands::queue::unchanged_active_queue_job,
            commands::queue::cancel_active_queue_job,
            commands::queue::cancel_queued_job,
            commands::queue::block_queued_jobs,
            commands::queue::clear_completed_queue_jobs,
            commands::trash_original,
            commands::activate_license,
            commands::forget_license
        ])
        .build(tauri::generate_context!())
        .expect("failed to build DropSquash desktop application");
    let mut did_run_startup_observation = false;
    app.run(move |handle, event| {
        if matches!(
            event,
            tauri::RunEvent::ExitRequested { .. } | tauri::RunEvent::Exit
        ) {
            commands::secure_share::recording::cancel_active(&handle.state());
        }
        if did_run_startup_observation || !matches!(event, tauri::RunEvent::Ready) {
            return;
        }
        did_run_startup_observation = true;
        commands::qa::secure_share::run_startup_observation(handle.clone());
    });
}
