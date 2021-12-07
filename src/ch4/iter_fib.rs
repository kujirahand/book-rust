// フィボナッチ数列を返すイテレータ --- (*1)
struct FibIterator {
    a: usize,
    b: usize,
}
impl FibIterator {
    fn new() -> Self { FibIterator {a: 1, b: 1} }
}
// イテレータを実装 --- (*2)
impl Iterator for FibIterator {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.a;
        self.a = self.b;
        self.b += tmp;
        return Some(self.a);
    }
}

fn main() {
    // forを使って結果を5個表示 --- (*3)
    let fib_iter = FibIterator::new();
    for (i, n) in fib_iter.enumerate() {
        if i >= 10 { break; }
        print!("{},", n);
    }
    println!("");
    // takeを使う場合 --- (*4)
    let fib_iter = FibIterator::new();
    fib_iter.take(10).for_each(|f| print!("{},", f));
}

