import random

# 読み込んだデータを代入するリスト
data = []

# 各ファイルを読み込んでdataに入れる --- (*1)
files = ["who.txt", "what.txt", "do.txt"]
for file in files:
    # ファイルを読む
    with open(file, "rt") as fp:
        s = fp.read().strip()
    # リストに読んだデータを分割してdataへ追加
    lines = s.split("\n")
    data.append(lines)

# 3回作文を行う --- (*2)
for i in range(3):
    words = []
    # who/what/doから単語を一つを選び出す
    for lines in data:
        w = lines[random.randint(0, len(lines)-1)]
        words.append(w)
    # 選んだ語句を出力
    print("{}が{}を{}。".format(words[0], words[1], words[2]))




