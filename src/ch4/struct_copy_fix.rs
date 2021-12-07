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
    // JiroはTaroを複製して名前だけ変えたい
    let jiro = Person {
        name: String::from("Jiro"),
        ..taro // 更新記法 --- (*1)
    };
    // TaroとJiroを表示
    println!("{},{}", taro.name, taro.age);
    println!("{},{}", jiro.name, jiro.age);
}

