# Pythonで英和辞書
import sys
dicfile = "ejdict-hand-utf8.txt"

# 引数をチェック
if len(sys.argv) < 2:
    print("[USAGE] jisyo.py word")
    quit()
# 指定された単語
word = sys.argv[1]

# 辞書データを一行ずつ調べる
with open(dicfile, "rt", encoding="utf-8") as fp:
    while True:
        line = fp.readline()
        if not line: break
        if word in line:
            print(line.strip())

