fn main() {
    for i in 1..=100 {
        // 値をmatchで分岐するがタプルを指定 --- (*1)
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"), // --- (*2)
            (0, _) => println!("Fizz"), // --- (*3)
            (_, 0) => println!("Buzz"),
            _      => println!("{}", i),
        }
    }
}

