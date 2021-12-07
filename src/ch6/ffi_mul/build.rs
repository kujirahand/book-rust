extern crate cc;

fn main() {
    // C言語のプログラムをコンパイル
    cc::Build::new()
        .file("src/mycalc.c") // C言語のプログラムを指定
        .include("src")
        .compile("mycalc"); // 出力ライブラリ名を指定
}
