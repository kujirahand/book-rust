// 問題のあるプログラム
struct Person {
    name: String,
    age: i32,
}
impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self { name: name.to_string(), age }
    }
}

fn main() {
    // Taroを作成
    let taro = Person::new("Taro", 18);
    // JiroはTaroを複製して名前だけ変えたい --- (*1)
    let mut jiro = taro;
    jiro.name = String::from("Jiro");
    // TaroとJiroを表示
    println!("{},{}", taro.name, taro.age); // ←エラー --- (*2)
    println!("{},{}", jiro.name, jiro.age);
}

