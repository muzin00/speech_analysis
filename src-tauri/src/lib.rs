mod commands;
mod crates {
    pub mod python;
    pub mod recorder;
}

pub use crates::{python, recorder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    python::setup();

    let recorder = recorder::Recorder::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(recorder)
        .invoke_handler(tauri::generate_handler![
            commands::start_recording,
            commands::stop_recording,
            commands::send_recording_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
