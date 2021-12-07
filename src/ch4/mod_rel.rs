mod random {
    pub mod linear {
        pub fn rand() -> u32 { // --- (*a)
            1
        }
    }
    pub mod xorshift {
        pub fn rand() -> u32 { // --- (*b)
            // (*a)の関数を呼ぶには？
            super::linear::rand()
        }
    }
}

fn main() {
    // (*b)のrandを呼ぶ
    println!("{}", random::xorshift::rand());
}


