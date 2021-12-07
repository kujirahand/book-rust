use hound;
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
        "pulse.wav", spec).unwrap();
    // 波形をwavに追加する
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    // Duty比を変えてパルス波を生成 --- (*1)
    [0.3, 0.1, 0.7, 0.5].iter().for_each(|duty| {
        [60,64,67,72].iter().for_each(|no| {
            wav.extend(pulse(
                *no, calc_len(bpm, 4), 0.5, *duty));
        });
    });
    // ファイルに書き込む
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}
fn noteno_to_hz(no: i32) -> f32 {
     440.0 * 2.0f32.powf((no-69) as f32 / 12.0)
}
fn calc_len(bpm: usize, n: usize) -> usize {
    ((4.0 / n as f32) * 
     (60.0 / bpm as f32) * SAMPLE_RATE) as usize
}
// パルス波を生成する --- (*2)
fn pulse(no: i32, len: usize, gain: f32, duty: f32) -> Vec<f32> {
    let tone = noteno_to_hz(no); // 周波数
    let form_samples = SAMPLE_RATE / tone; // 周期
    let mut wav:Vec<f32> = vec![0.0; len];
    for i in 0..len {
        let saw = (i as f32 / form_samples) % 1.0;
        wav[i] = if saw > duty { -1.0 } else { 1.0 };
    }
    // 音量を調節する
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}


