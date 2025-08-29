use crate::python::{create_wav_file, RecordingData};
use crate::recorder::Recorder;
use tauri::State;

#[tauri::command]
pub fn start_recording(recorder: State<Recorder>) -> Result<(), String> {
    recorder.start()?;
    Ok(())
}

#[tauri::command]
pub fn stop_recording(recorder: State<Recorder>) -> Result<(), String> {
    recorder.stop()?;
    Ok(())
}

#[tauri::command]
pub fn send_recording_data(recorder: State<Recorder>) -> Result<(), String> {
    create_wav_file(RecordingData {
        channels: recorder.channels,
        sample_rate: recorder.sample_rate,
        samples: recorder.data().unwrap(),
    });
    Ok(())
}
