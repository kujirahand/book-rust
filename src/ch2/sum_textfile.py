import sys
total = 0
# コマンドラインに指定したファイルを全て処理
for i, v in enumerate(sys.argv):
    if i == 0: continue
    # テキストファイルを読む
    with open(v, "rt") as fp:
        text = fp.read()
        # 改行で区切って読む
        for line in text.split("\n"):
            try:
                total += float(line)
            except ValueError:
                pass;
# 結果を表示
print(total)

