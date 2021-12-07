use std::f32::consts::PI;
use std::io::{Write, Seek};
use hound::WavWriter;
// 定数
const SAMPLE_RATE: f32 = 44100.0;
const BPM: f32 = 120.0; // BMP(テンポ)

#[allow(unused_variables)]
fn main() {
    // WavWriterオブジェクトを生成 --- (*1)
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut fw = WavWriter::create("melody.wav", spec).unwrap();
    // 音階を定義 --- (*2)
    let (c4, d4, e4, f4) = (261.626, 293.665, 329.628, 349.228);
    let (g4, a4, b4, c5) = (391.995, 440.000, 493.883, 523.251);
    // 音長を定義 --- (*3)
    let l4 = ((60.0 / BPM) * SAMPLE_RATE) as u32;
    let l2 = l4 * 2;
    // メロディーを指定 --- (*4)
    write_tone(&mut fw, c4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, e4, l2);
    write_tone(&mut fw, c4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, e4, l2);
    write_tone(&mut fw, g4, l4);
    write_tone(&mut fw, e4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, c4, l4);
    write_tone(&mut fw, d4, l4);
    write_tone(&mut fw, e4, l4);
    write_tone(&mut fw, d4, l2);
}

// 指定の音階を指定の長さ分書き込む --- (*5)
fn write_tone<W>(fw: &mut WavWriter<W>, tone: f32, len: u32) 
where W: Write + Seek {
    for t in 0..len {
        let a = t as f32 / SAMPLE_RATE;
        let v = (a * tone * 2.0  * PI).sin();
        fw.write_sample((v * i16::MAX as f32) as i16).unwrap();
    }
}

