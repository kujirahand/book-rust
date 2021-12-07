// CarSpec構造体を定義 --- (*1)
struct CarSpec {
    model: i32, // 型番
    cc: i32, // 排気量
    color: i32, // 色
}

fn main() {
    // CarSpecオブジェクトを生成 --- (*2)
    let car1 = CarSpec {
        model: 3001,
        cc: 1500,
        color: 0xFF0000,
    };
    let car2 = CarSpec {
        model: 3002,
        cc: 1200,
        color: 0x0000FF,
    };
    // オブジェクトのフィールドを表示 --- (*3)
    println!("car1: {}, {}cc, {:06x}", 
        car1.model, car1.cc, car1.color);
    println!("car2: {}, {}cc, {:06x}", 
        car2.model, car2.cc, car2.color);
}

