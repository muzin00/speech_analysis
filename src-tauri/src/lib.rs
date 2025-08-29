mod commands {
    pub mod recording;
}

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
            commands::recording::start_recording,
            commands::recording::stop_recording,
            commands::recording::send_recording_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
