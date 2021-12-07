use std::f32::consts::PI;
pub const SAMPLE_RATE: f32 = 44100.0;
// ノートを表す構造体を定義 --- (*1)
pub struct Note {
    pub no: i32, // ノート番号
    pub len: isize, // 音長
    pub gain: f32, // 音量
    pub params: (f32, f32), // 音色のパラメータ
}
// FM方式の波形を生成する --- (*2)
pub fn write(track: &mut Vec<f32>, note: Note) {
    let tone = noteno_to_hz(note.no); // 周波数
    let a = 2.0 * PI * tone / SAMPLE_RATE;
    let len = note.len as usize;
    let mut wav:Vec<f32> = vec![0.0; len];
    // サイン波を書き出す --- (*3)
    for i in 0..(note.len as usize) {
        let t = i as f32;
        let sin1 = note.params.0 * (a * t).sin();
        let sin2 = note.params.1 * (a * t + sin1).sin();
        let sin3 = (a * t + sin2).sin();
        wav[i] = sin3;
    }
    // 音量の調整(ADSR) --- (*4)
    let a = 3; // Attack(立ち上がり)
    let d = a + 200; // Decay(減衰)
    let s = 0.90; // Sustain(減衰後の維持)
    let r = (len as f32 * 0.4) as usize; // Release(余韻)
    let w:Vec<f32> = wav.into_iter().enumerate().map(|(i, v)| {
        let v = (v * note.gain) as f32;
        if i < a { return i as f32 / a as f32 * v; }
        let v2 = v * s;
        if i < d {
            let dec = (1.0 - (i as f32 / d as f32)) * 
                (1.0 - s) * v.abs();
            return if v2 > 0.0 { v + dec } else { v - dec }
        }
        else if i > (len - r) {
            let i2 = i - (len - r);
            return (1.0 - i2 as f32 / r as f32) * v2
        }
        v2
    }).collect();
    track.extend(w);
}
pub fn calc_len(bpm: usize, n: usize) -> isize {
    ((4.0 / n as f32) * 
     (60.0 / bpm as f32) * SAMPLE_RATE) as isize
}
fn noteno_to_hz(no: i32) -> f32 {
     440.0 * 2.0f32.powf((no-69) as f32 / 12.0)
}

