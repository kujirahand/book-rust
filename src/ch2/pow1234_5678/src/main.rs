// BigIntを使うための宣言 --- (*1)
use num_bigint::BigInt;

fn main() {
    // BigIntのオブジェクトを作って値を設定 --- (*2)
    let v = BigInt::from(1234);
    // 5678乗を計算 --- (*3)
    println!("{}", v.pow(5678));
}

