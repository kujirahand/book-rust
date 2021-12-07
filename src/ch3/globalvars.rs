// グローバル変数を定義 --- (*1)
static mut TAX: f32 = 0.1;

fn main() {
    // 安全でないことを宣言 --- (*2)
    unsafe {
        // ミュータブルなstatic変数を使う --- (*3)
        println!("Price: {}", TAX * 300.0);
        // staticな変数を変更する --- (*4)
        TAX = 0.08;
        println!("Price: {}", TAX * 300.0);
    }
}

