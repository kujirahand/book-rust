use crate::wav_writer::Note; // --- (*1)

// MMLを解析してVec<Note>に変換 --- (*2)
pub fn parse(src: String) -> Vec<Note> {
    // 解析結果を初期化
    let mut result = vec![];
    // 音階と音長の初期値を指定 --- (*3)
    let mut octave = 5;
    let mut length = 4;
    // 文字列を1文字ずつ順に読んでいく --- (*4)
    let mut it = src.chars();
    while let Some(ch) = it.next() {
        // コマンドで処理を分岐 --- (*5)
        match ch {
            'a'..='g' => { // ノート --- (*6)
                let note = match ch {
                    'c' => 0, 'd' => 2, 'e' => 4, 'f' => 5, 
                    'g' => 7, 'a' => 9, 'b' => 11, _ => 0,
                };
                let no = note + octave * 12;
                result.push(Note{no, len: length});
            },
            // 休符の場合
            'r' => result.push(Note{no: -1, len: length}),
            'o' => { // オクターブ(音階) --- (*7)
                let v = it.next().expect("oの後は数値");
                let o = v as i32 - '0' as i32;
                if o >= 0 && o < 9 { octave = o; }
            },
            'l' => { // 音長 --- (*8)
                let v = it.next().expect("lの後は数値");
                let l = v as i32 - '0' as i32;
                if l >= 1 && l <= 9 { length = l; }
            },
            _ => {}, // それ以外の文字は飛ばす
        };
    }
    result
}

// テスト --- (*9)
#[cfg(test)]
mod mml_parser_test {
    use super::*; // 外側の要素を利用
    #[test]
    fn parse_test() {
        let mml = "l2 o5 cde".to_string();
        let notes = parse(mml);
        assert_eq!(notes[0].no, 60);
        assert_eq!(notes[0].len, 2);
        assert_eq!(notes[1].no, 62);
        assert_eq!(notes[2].no, 64);
    }
}
