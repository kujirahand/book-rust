# Pythonでコインのお釣り枚数を計算
# 期待する金額
price = 3950
# 500円玉の枚数を繰り返し調べる
for i500 in range(0, 11):
    # 100円玉の枚数を繰り返し調べる
    for i100 in range(0, 4):
        # 50円玉の枚数を繰り返し調べる
        for i50 in range(0, 11):
            # 総額を計算
            total = i50 * 50 + i100 * 100 + i500 * 500
            # 総額が期待する金額になるか
            if price == total:
                print("500円x{}+100円x{}+50円x{}={}"
                        .format(i500,i100,i50,total))

