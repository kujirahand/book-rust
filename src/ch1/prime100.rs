// Rustで素数を100個生成

// 素数判定をする関数 --- (*1)
fn is_prime(n: usize) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false 
        }
    }
    return true
}

// 素数を100個求める関数 --- (*2)
fn get_primes(primes: &mut[usize; 100]) {
    let mut i = 2;
    let mut count = 0;
    // countが100に達するまで繰り返す --- (*3)
    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main() {
    // 初期値0の配列を100個用意する --- (*4)
    let mut primes = [0; 100];
    // 素数を100個求める --- (*5)
    get_primes(&mut primes);
    // 結果を表示 --- (*6)
    println!("{:?}", primes);
}

