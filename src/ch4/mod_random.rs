// randomモジュールを宣言 --- (*1)
mod random {
    // linearモジュールを宣言 --- (*2)
    pub mod linear {
        use std::num::Wrapping;
        // 線形合同法で乱数を生成 --- (*3)
        pub fn rand(seed: &mut u32) -> u32 {
            let (a, c) = (134775813u32, 12345u32);
            *seed = (Wrapping(*seed) * Wrapping(a) +
                     Wrapping(c)).0;
            *seed
        }
    }
    // xorshift モジュールを宣言 --- (*4)
    pub mod xorshift {
        // XorShiftで乱数を生成 --- (*5)
        pub fn rand(seed: &mut u32) -> u32 {
            let mut y = *seed;
            y ^= y << 13;
            y ^= y >> 17;
            y ^= y << 5;
            *seed = y;
            y
        }
    }
}

// モジュールの利用を宣言 --- (*6)
use random::{linear, xorshift};
fn main() {
    // それぞれ10回乱数を生成して表示 --- (*7)
    let mut seed1 = 12345u32;
    let mut seed2 = 12345u32;
    for i in 0..10 {
        let r1 = linear::rand(&mut seed1) % 6 + 1;
        let r2 = xorshift::rand(&mut seed2) % 6 + 1;
        println!("L:{}回目= {}, {}", i, r1, r2);
    }
}

