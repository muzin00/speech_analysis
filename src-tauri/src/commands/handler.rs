use crate::commands;
use crate::recorder::Recorder;
use tauri::ipc::Request;
use tauri::AppHandle;
use tauri::Manager;

#[tauri::command]
pub fn root_handler(request: Request, app_handle: AppHandle) -> Result<(), String> {
    let Some(path) = request.headers().get("Path") else {
        return Err("Path header not found or invalid".to_string());
    };

    let recorder = app_handle.state::<Recorder>();

    match path.to_str().unwrap() {
        "start_recording" => commands::recording::start_recording(recorder),
        "stop_recording" => commands::recording::stop_recording(recorder),
        "send_recording_data" => commands::recording::send_recording_data(recorder),
        _ => return Err("Invalid path".to_string()),
    }
}
