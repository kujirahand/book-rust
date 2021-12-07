// 硬貨の種類を表す列挙型 --- (*1)
enum Coin {
    Coin1(isize),
    Coin5(isize),
    Coin10(isize),
    Coin50(isize),
    Coin100(isize),
    Coin500(isize),
}
impl Coin {
    // 硬貨の種類から実際の金額を計算 --- (*2)
    fn calc_price(&self) -> isize {
        match *self {
            Coin::Coin1(v) => v,
            Coin::Coin5(v) => v * 5,
            Coin::Coin10(v) => v * 10,
            Coin::Coin50(v) => v * 50,
            Coin::Coin100(v) => v * 100,
            Coin::Coin500(v) => v * 500,
        }
    }
}

fn main() {
    // 財布の中にある硬貨の種類と枚数を指定 --- (*3)
    let wallet: Vec<Coin> = vec![
        Coin::Coin1(3), // 1円が3枚
        Coin::Coin5(10), // 5円が10枚
        Coin::Coin10(5), // 10円がが5枚
        Coin::Coin50(1), // 50円が1枚
        Coin::Coin100(1), // 100円が1枚
        Coin::Coin500(5), // 500円が5枚
    ];
    // 金額を計算して表示 --- (*4)
    let total = wallet.iter()
        .fold(0, |sum, v| sum + v.calc_price());
    println!("財布の合計は{}円です", total);
}

