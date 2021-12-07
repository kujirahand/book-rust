// 配列をシャッフルするのに必要な宣言 --- (*1)
use rand::seq::SliceRandom;

fn main() {
    // 1から75までの数値を配列に代入 --- (*2)
    let mut nums = [0; 75];
    for i in 1..=75 { nums[i-1] = i; }
    
    // シャッフル --- (*3)
    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    // カードを表示 --- (*4)
    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 { // ワイルドカードの時
                print!("  *,");
            } else {
                print!("{:3},", nums[i]);
            }
        }
        println!("");
    }
}
