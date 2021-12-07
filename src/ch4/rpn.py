# PythonでRPN電卓
# 計算用のスタックを準備
stack = []
# 標準入力から数式を得る
s = input('RPN: ')
# 式を空白で分割して繰り返し計算
tokens = s.split()
for t in tokens:
    t = t.strip()
    try:
        # 数値ならスタックにPUSH
        stack.append(float(t))
    except ValueError:
        # 演算子なら2回POPして計算結果をPUSH
        b = stack.pop()
        a = stack.pop()
        if   t == "+":  stack.append(a + b)
        elif t == "-":  stack.append(a - b)
        elif t == "*":  stack.append(a * b)
        elif t == "/":  stack.append(a / b)
        else: raise Exception("未定義の演算子:" + t)
# 計算結果を表示
print(stack.pop())

