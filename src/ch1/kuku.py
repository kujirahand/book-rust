# Pythonで九九の表を作成
for y in range(1, 10):
    for x in range(1, 10):
        print("{:3},".format(y * x), end="")
    print("")

