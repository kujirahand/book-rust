// 問題のあるプログラム
fn main() {
    let target = "aaa,bbb,ccc";
    let lines:Vec<String> = target.split(",").collect();
    println!("{:?}", lines);
}

