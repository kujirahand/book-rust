fn main() {
    // 文字列を生成
    let mut s = String::from("能ある鷹は爪を隠す");
    // 連続で可変参照を使うにはスコープを分割する
    {
        let ref1 = &mut s;
        println!("ref1={}", ref1);
    } // ここでref1は破棄される
    {
        let ref2 = &mut s;
        println!("ref2={}", ref2);
    }
}

