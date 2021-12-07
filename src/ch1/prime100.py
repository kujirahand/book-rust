# Pythonで素数を100個生成

# 素数判定を行う関数 --- (*1)
def is_prime(n):
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

# count個の素数を生成する --- (*2)
def get_primes(count):
    res = []
    i = 2
    while len(res) < count:
        if is_prime(i):
            res.append(i)
        i += 1
    return res

# 100個の素数を表示
print(get_primes(100))

