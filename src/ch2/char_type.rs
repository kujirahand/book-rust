fn main() {
    let a = 'a';  // 文字'a'を指定
    let b = b'a'; // ASCIIコード 97u8 を指定
    let c = '\x61'; // 16進数で文字'a'を指定
    println!("{},{:2x},{}", a, b, c);

    let d = '愛'; // 文字'愛'を指定
    let e = '愛' as u32; // 文字'愛'の文字コード0x611bを指定
    let f = '\u{611b}'; // 16進数で文字'愛'を指定
    println!("{},{:4x},{}", d, e, f);
}

