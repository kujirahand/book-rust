mod fm;
use hound;
fn main() {
    // WavWriterを生成
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: fm::SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create(
        "fm.wav", spec).unwrap();
    // 波形をtrackに追加する --- (*1)
    let mut track: Vec<f32> = vec![];
    let bpm = 120;
    let len = fm::calc_len(bpm, 4);
    // 音色のパラメータを変えながら演奏 --- (*2)
    let params = [(4.5,2.0),(7.0,3.0),(3.0,2.0),(11.0,4.0)];
    for p in params {
        for note_no in [60,64,67,64, 60,64,67,72] {
            fm::write(&mut track, fm::Note {
                no: note_no, len, gain: 0.5, params: p
            });
        }
    }
    // ファイルに書き込む
    for v in track.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}


