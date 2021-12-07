// 素数を返す8ビットイテレータ
struct PrimeIterator {
    n: u8,
}
// メソッドを定義 --- (*1)
impl PrimeIterator {
    fn new() -> Self { PrimeIterator {n: 1} }
    // self.nが素数かどうか調べる
    fn is_prime(&self) -> bool {
        for i in 2..self.n {
            if self.n % i == 0 { return false; }
        }
        return true;
    }
}
// イテレータを実装 --- (*2)
impl Iterator for PrimeIterator {
    type Item = u8; // --- (*3)
    // 次の要素を返す --- (*4)
    fn next(&mut self) -> Option<Self::Item> {
        // 素数を探して返す
        loop {
            self.n += 1;
            // 8ビットを超える素数を調べない --- (*5)
            if std::u8::MAX == self.n {
                return None
            }
            // self.n が素数か調べる --- (*6)
            if self.is_prime() { return Some(self.n); }
        }
    }
}

fn main() {
    // イテレータを生成 --- (*7)
    let prime_iter = PrimeIterator::new();
    // for文で繰り返す
    for n in prime_iter {
        print!("{},", n);
    }
}
