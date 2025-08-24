use crate::python;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, StreamConfig,
};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum RecordCommand {
    Start,
    Stop,
    SendData,
}

pub struct RecordingData {
    pub channels: u16,
    pub sample_rate: u32,
    pub samples: Vec<f32>,
}

pub struct RecordingThread {
    _handle: thread::JoinHandle<()>,
    command_sender: mpsc::Sender<RecordCommand>,
}

impl RecordingThread {
    pub fn new() -> Self {
        let (command_sender, command_receiver) = mpsc::channel();
        let command_receiver = Arc::new(Mutex::new(command_receiver));

        let handle = thread::spawn(move || {
            let host = cpal::default_host();
            let device = host
                .default_input_device()
                .expect("入力デバイスが見つかりません");

            let supported_config = device
                .supported_input_configs()
                .unwrap()
                .next()
                .expect("サポートされている構成がありません")
                .with_max_sample_rate();

            let config: StreamConfig = supported_config.into();

            let mut stream: Option<Stream> = None; // ストリームを保持
            let mut audio_buffer: Vec<f32> = Vec::new(); // ここでデータを保持
            let (data_sender, data_receiver) = mpsc::channel::<Vec<f32>>();

            loop {
                // CPALからデータを受け取る
                while let Ok(data) = data_receiver.try_recv() {
                    audio_buffer.extend_from_slice(&data);
                }

                // コマンドを受け取る
                match command_receiver.lock().unwrap().try_recv() {
                    Ok(RecordCommand::Start) => {
                        println!("Recorder: 録音開始コマンドを受信。");
                        let data_sender = data_sender.clone();
                        stream = Some(
                            device
                                .build_input_stream(
                                    &config,
                                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                                        let _ = data_sender.send(data.to_vec());
                                    },
                                    |err| eprintln!("an error occurred on stream: {}", err),
                                    None,
                                )
                                .unwrap(),
                        );

                        if let Some(stream) = &stream {
                            stream.play().unwrap();
                        }
                    }
                    Ok(RecordCommand::Stop) => {
                        println!("Recorder: 録音停止コマンドを受信。");
                        if let Some(stream) = &stream {
                            stream.pause().unwrap();
                        }
                    }
                    Ok(RecordCommand::SendData) => {
                        println!("Recorder: データ送信コマンドを受信。");
                        python::create_wav_file(RecordingData {
                            channels: config.channels as u16,
                            sample_rate: config.sample_rate.0,
                            samples: audio_buffer.clone(),
                        });
                    }
                    Err(mpsc::TryRecvError::Disconnected) => break,
                    _ => {
                        // 一定サイズに達したら自動的に送信
                    }
                }
            }
        });

        Self {
            _handle: handle,
            command_sender,
        }
    }

    pub fn start(&self) -> Result<(), String> {
        self.command_sender
            .send(RecordCommand::Start)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn stop(&self) -> Result<(), String> {
        self.command_sender
            .send(RecordCommand::Stop)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn send_data(&self) -> Result<(), String> {
        self.command_sender
            .send(RecordCommand::SendData)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
