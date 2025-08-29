use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, StreamConfig,
};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

enum RecordCommand {
    Start,
    Stop,
}

pub struct Recorder {
    pub channels: u16,
    pub sample_rate: u32,
    audio_buffer: Arc<Mutex<Vec<f32>>>,
    command_sender: mpsc::Sender<RecordCommand>,
    _handle: thread::JoinHandle<()>,
}

impl Recorder {
    pub fn new() -> Self {
        let (command_sender, command_receiver) = mpsc::channel();
        let command_receiver = Arc::new(Mutex::new(command_receiver));
        let audio_buffer = Arc::new(Mutex::new(Vec::new()));
        let audio_buffer_clone = audio_buffer.clone();

        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("入力デバイスが見つかりません");

        let supported_config = device
            .default_input_config()
            .expect("入力デバイス設定が見つかりません");

        let config: StreamConfig = supported_config.into();
        let channels = config.channels as u16; // チャンネル数
        let sample_rate = config.sample_rate.0; // サンプルレート

        let handle = thread::spawn(move || {
            let mut stream: Option<Stream> = None; // ストリームを保持
            let (data_sender, data_receiver) = mpsc::channel::<Vec<f32>>();

            loop {
                // CPALからデータを受け取る
                while let Ok(data) = data_receiver.try_recv() {
                    let mut audio_buffer = audio_buffer_clone.lock().unwrap();
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
                    Err(mpsc::TryRecvError::Disconnected) => break,
                    _ => {
                        // 一定サイズに達したら自動的に送信
                    }
                }
            }
        });

        Self {
            channels,
            sample_rate,
            audio_buffer,
            command_sender,
            _handle: handle,
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

    pub fn data(&self) -> Result<Vec<f32>, String> {
        let audio_buffer = self.audio_buffer.lock().unwrap();
        Ok(audio_buffer.clone())
    }
}
