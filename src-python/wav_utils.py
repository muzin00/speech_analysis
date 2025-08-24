import wave
import struct
from typing import TypedDict, Dict, Any

class WavParams(TypedDict):
    channels: int
    sample_rate: int
    samples: list[float]

def create_wav_file(params: Dict[str, Any]):
    """WAVファイルを作成する関数
    
    Args:
        params: チャンネル数、サンプルレート、サンプルデータを含む辞書
    """
    samples = params.get('samples', [])
    channels = params.get('channels', 1)
    sample_rate = params.get('sample_rate', 44100)
    
    print(f"Creating WAV: {len(samples)} samples, {channels} channels, {sample_rate}Hz")
    
    # デバッグ情報を追加
    if samples:
        print(f"Sample range: min={min(samples):.6f}, max={max(samples):.6f}")
        print(f"Max absolute value: {max(abs(sample) for sample in samples):.6f}")
    
    # f32の範囲を16ビット整数の範囲（-32768 から 32767）に安全に正規化
    if not samples:
        normalized_samples = []
    else:
        # 絶対値の最大値を取得
        max_abs = max(abs(sample) for sample in samples)
        
        if max_abs > 0:
            # 音声増幅処理 - 録音レベルが低い場合に自動増幅
            target_max = 0.8  # 目標最大値（-1.0〜1.0の80%）
            
            if max_abs < target_max:
                # 録音レベルが低い場合、増幅
                amplification_factor = target_max / max_abs
                # 増幅しすぎないように制限（最大10倍）
                amplification_factor = min(amplification_factor, 10.0)
                scale_factor = 32767.0 * amplification_factor
                print(f"Amplifying audio: factor={amplification_factor:.2f}x, scale_factor={scale_factor:.2f}")
            elif max_abs <= 1.0:
                # 適切な範囲内なら、そのまま使用
                scale_factor = 32767.0
                print(f"Using direct scaling (max_abs <= 1.0): {scale_factor}")
            else:
                # 1.0を超える場合のみ正規化
                scale_factor = 32767.0 / max_abs
                print(f"Using normalization (max_abs > 1.0): scale_factor = {scale_factor:.6f}")
            
            normalized_samples = [int(sample * scale_factor) for sample in samples]
            print(f"Normalized range: min={min(normalized_samples)}, max={max(normalized_samples)}")
        else:
            normalized_samples = [0] * len(samples)
            print("All samples are zero")
    
    # 16ビット整数をバイトデータに変換
    audio_data = struct.pack(f'<{len(normalized_samples)}h', *normalized_samples)
    
    # WAVファイルを作成
    with wave.open('../wave_output.wav', 'wb') as wf:
        wf.setnchannels(channels)
        wf.setsampwidth(2)  # 16ビット
        wf.setframerate(sample_rate)
        wf.writeframes(audio_data)
    
    print(f"WAV file created: wave_output.wav ({len(audio_data)} bytes)")