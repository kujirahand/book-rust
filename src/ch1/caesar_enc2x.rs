// Rustでシーザー暗号に変換する関数（その2）
fn encrypt(text: &str, shift: i16) -> String {
    let a = 'A' as i16;
    let conv = |ch| (((ch+a+shift+26)%26+a) as u8)as char;
    let is_alpha = |ch| 'A' <= ch && ch <= 'Z';
    text.chars()
        .map(|c| if is_alpha(c) { conv(c as i16) } else { c })
        .collect()
}

fn main() {
    let enc = encrypt("I LOVE YOU.", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec);
}

