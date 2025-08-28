use crate::python::{create_wav_file, RecordingData};
use crate::recording::RecordingThread;
use tauri::State;

#[tauri::command]
pub fn start_recording(recording_thread: State<RecordingThread>) -> Result<(), String> {
    recording_thread.start()?;
    Ok(())
}

#[tauri::command]
pub fn stop_recording(recording_thread: State<RecordingThread>) -> Result<(), String> {
    recording_thread.stop()?;
    Ok(())
}

#[tauri::command]
pub fn send_recording_data(recording_thread: State<RecordingThread>) -> Result<(), String> {
    create_wav_file(RecordingData {
        channels: recording_thread.channels,
        sample_rate: recording_thread.sample_rate,
        samples: recording_thread.data().unwrap(),
    });
    Ok(())
}
