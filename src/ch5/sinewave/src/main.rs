use std::f32::consts::PI;
use hound::WavWriter;

// 定数を宣言 --- (*1)
const SAMPLE_RATE: u32 = 44100;
const TONE: f32 = 440.0; // 440Hz = A

fn main() {
    // WAVファイルのフォーマットを指定 --- (*2)
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    // WavWriterのオブジェクトを生成 --- (*3)
    let mut fw = WavWriter::create("a.wav", spec).unwrap();
    // 連続でサイン波を書き込む --- (*4)
    let samples = SAMPLE_RATE * 3; // 3秒
    for t in 0 .. samples {
        let v = ((t as f32 / SAMPLE_RATE as f32) * 
                 TONE * 2.0 * PI).sin();
        fw.write_sample((v * i16::MAX as f32) as i16).unwrap();
    }
}

