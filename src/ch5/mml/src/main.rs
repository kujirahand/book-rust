mod mml_parser;
mod wav_writer;

fn main() {
    // カエルの歌の楽譜 --- (*1)
    let mml = format!("{}{}",
            "o5 l4 cdef edl2c l4 efga gfl2e",
            "l4 crcr crcr l8 ccdd eeff l4 ed l2c");
    // 楽譜を一音ずつのVec<Note>に変換 --- (*2)
    let notes = mml_parser::parse(mml);
    // WAVファイルへ書き込む --- (*3)
    let bpm = 120.0;
    wav_writer::write("kaeru.wav", notes, bpm);

    // キラキラ星の楽譜 --- (*4)
    let mml = format!("{}{}{}",
        "o5 l4 ccgg aal2g l4 ffee ddl2c",
        "l4 ggff eel2d l4 ggff eel2d",
        "l4 ccgg aal2g l4 ffee ddl2c"
    );
    let notes = mml_parser::parse(mml);
    let bpm = 120.0;
    wav_writer::write("kirakira.wav", notes, bpm);
}

