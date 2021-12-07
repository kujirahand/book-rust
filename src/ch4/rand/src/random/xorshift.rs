// XorShiftで疑似乱数を生成する
pub fn rand(seed: &mut u32) -> u32 {
    let mut y = *seed;
    y ^= y << 13;
    y ^= y >> 17;
    y ^= y << 5;
    *seed = y;
    y
}

