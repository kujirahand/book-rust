use hound;
use rand::prelude::*;
const SAMPLE_RATE: f32 = 44100.0;
fn main() {
    // WavWriterを生成
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create(
        "noise.wav", spec).unwrap();
    // ノイズを生成
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    // 生成 --- (*1)
    // フルレンジのノイズを生成
    wav.extend(noise(2.0, -1.0, calc_len(bpm, 2)));
    // 0.8から1.0 のノイズを生成
    wav.extend(noise(0.2, 0.8, calc_len(bpm, 2)));
    // -1.0から-0.2のノイズを生成
    wav.extend(noise(0.8, -1.0, calc_len(bpm, 2)));
    // ファイルに書き込む
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}
fn calc_len(bpm: usize, n: usize) -> usize {
    ((4.0 / n as f32) * 
     (60.0 / bpm as f32) * SAMPLE_RATE) as usize
}
// ノイズを生成する --- (*2)
fn noise(range: f32, shift: f32, len: usize) -> Vec<f32> {
    let mut wav:Vec<f32> = vec![0.0; len];
    let mut rng = rand::thread_rng();
    for i in 0..len {
        wav[i] = rng.gen::<f32>() * range + shift;
    }
    // 音量を調整
    let gain = 0.5;
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}


