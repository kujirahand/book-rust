use std::num::Wrapping;

// 線形合同法で疑似乱数を生成
pub fn rand(seed: &mut u32) -> u32 {
    let (a, c) = (134775813u32, 12345u32);
    *seed = (Wrapping(*seed) * Wrapping(a) +
             Wrapping(c)).0;
    *seed
}


