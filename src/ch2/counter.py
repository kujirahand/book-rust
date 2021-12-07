# Pythonで人気投票の集計
# 投票データ --- (*1)
V_DATA = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C"
# 集計用の辞書型データを初期化 --- (*2)
c_dic = {"A": 0, "B": 0, "C": 0}
# 投票データをカウント --- (*3)
for w in V_DATA.split(","):
    c_dic[w] += 1
# 投票を集計して結果を表示 --- (*4)
for key in ["A", "B", "C"]:
    print("{}: {:2d}".format(key, c_dic[key]))

