use pyo3::prelude::*;
use pyo3::types::PyDict;

pub struct RecordingData {
    pub channels: u16,
    pub sample_rate: u32,
    pub samples: Vec<f32>,
}

pub fn setup() {
    // Pythonインタープリタを初期化
    pyo3::prepare_freethreaded_python();

    Python::with_gil(|py| {
        let sys = py.import("sys").unwrap();
        let current_dir = std::env::current_dir().unwrap();
        let src_python_path = current_dir.parent().unwrap().join("src-python");

        let path = sys.getattr("path").unwrap();
        let _ = path.call_method1("insert", (0, src_python_path.to_str().unwrap()));
    });
}

pub fn create_wav_file(recording_data: RecordingData) {
    Python::with_gil(|py| {
        let wav = py.import("wav_utils").unwrap();
        let params = PyDict::new(py);
        params
            .set_item("channels", recording_data.channels)
            .unwrap();
        params
            .set_item("sample_rate", recording_data.sample_rate)
            .unwrap();
        params.set_item("samples", recording_data.samples).unwrap();
        wav.call_method1("create_wav_file", (params,)).unwrap();
    });
}
