mod commands;
mod recording;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let recording_thread = recording::RecordingThread::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(recording_thread)
        .invoke_handler(tauri::generate_handler![
            commands::start_recording,
            commands::stop_recording,
            commands::send_recording_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
