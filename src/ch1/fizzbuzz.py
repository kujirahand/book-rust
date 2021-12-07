# PythonでFizzBuzz問題を解く
# 1から100まで繰り返す --- (*1)
for i in range(1, 101):
    # 条件を一つずつ判定する --- (*2)
    if i % 3 == 0 and i % 5 == 0:
        print("FizzBuzz")
    elif i % 3 == 0:
        print("Fizz")
    elif i % 5 == 0:
        print("Buzz")
    else:
        print(i)

