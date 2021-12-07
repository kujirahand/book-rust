use std::fs;
use rand::Rng;

fn main() {
    // 読み込んだデータを代入するベクタを準備 --- (*1)
    let mut data: Vec<Vec<String>> = vec![];
    // 各ファイルを読む --- (*2)
    let files = ["who.txt", "what.txt", "do.txt"];
    for file in files {
        // ファイルを読む --- (*3)
        let s = fs::read_to_string(file).expect("読込エラー");
        // 区切ってデータに代入 --- (*4)
        let mut lines: Vec<String> = vec![];
        for line in s.trim().split("\n") {
            lines.push(String::from(line));
        }
        data.push(lines);
    }
    // 乱数生成器を準備 --- (*5)
    let mut rng = rand::thread_rng();
    // 3回作文する --- (*6)
    for _ in 0..3 {
        // who/what/doから1語ずつ単語を選ぶ --- (*7)
        let mut words = vec![];
        for lines in data.iter() {
            let i = rng.gen_range(0..lines.len());
            let w = &lines[i];
            words.push(w);
        }
        // 作文を出力 --- (*8)
        println!("{}が{}を{}。", words[0], words[1], words[2]);
    }
}
