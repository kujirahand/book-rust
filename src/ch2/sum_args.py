import sys
total = 0
# コマンドライン引数を順に足す
for i, v in enumerate(sys.argv):
    if i == 0: continue # 0番目はコマンド自身
    try:
        # 文字列を数値に変換
        total += float(v)
    except ValueError:
        pass;
# 結果を表示
print(total)

