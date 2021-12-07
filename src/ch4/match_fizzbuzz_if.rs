fn main() {
    for i in 1..=100 {
        let msg = match i {
            n if n % 15 == 0 => "FizzBuzz".to_string(),
            n if n % 3 == 0 => "Fizz".to_string(),
            n if n % 5 == 0 => "Buzz".to_string(),
            _ => format!("{}", i),
        };
        println!("{}", msg);
    }
}

