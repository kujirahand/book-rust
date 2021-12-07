use portaudio as pa;
use std::f64::consts::PI;

// サウンド設定
const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: u32 = 44100;

fn main() {
    match play() {
        Ok(_) => {},
        Err(e) => { eprintln!("err:{:?}", e); }
    }
}

fn play() -> Result<(), pa::Error> {
    // PortAudioのオブジェクトを作成 --- (*1)
    let pa = pa::PortAudio::new()?;
    let mut settings = pa.default_output_stream_settings(
        CHANNELS, SAMPLE_RATE as f64, SAMPLE_RATE)?;
    settings.flags = pa::stream_flags::CLIP_OFF;
    // 再生のためのバッファを用意するクロージャ --- (*2)
    let tone = 440.0; // 440Hz = A
    let mut phase = 0;
    let callback = 
        move|pa::OutputStreamCallbackArgs{buffer, frames,..}| {
        let mut i = 0;
        for _ in 0..frames {
            // サイン波を鳴らす --- (*3)
            let v = (phase as f64 * tone * 2.0 * PI / 
                     SAMPLE_RATE as f64).sin() as f32;
            buffer[i+0] = v; // 左チャンネル
            buffer[i+1] = v; // 右チャンネル
            i += 2;
            phase +=1;
        }
        pa::Continue
    };
    // ストリームを開く --- (*4)
    let mut stream = pa.open_non_blocking_stream(
        settings, callback)?;
    // 再生 --- (*5)
    println!("サイン波を再生");
    stream.start()?;
    pa.sleep(NUM_SECONDS * 1000);
    stream.stop()?;
    stream.close()?;
    Ok(())
}

