# 問題のあるPythonプログラム
# Counterクラスを定義 --- (*1)
class Counter:
    value = 0
    # 値を1加算するメソッド
    def inc(self):
        self.value += 1
        print("value=", self.value)

# Counterクラスを引数にとる関数 --- (*2)
def count(counter):
    counter.inc()

# 正しく利用する例 --- (*3)
a = Counter()
count(a)
count(a)

# 間違った利用例 --- (*4)
a = None
count(a)
