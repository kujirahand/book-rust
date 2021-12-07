fn main() {
    let age = 8;
    let age_str = match age {
        0 => "乳児",
        1..=5 => "幼児",
        6..=11 => "こども",
        _ => "おとな",
    };
    println!("{}才は{}料金", age, age_str);
}

