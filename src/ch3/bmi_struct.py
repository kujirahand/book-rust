# PythonでBMI肥満度判定ツール(判定表を使ったもの)

# 身長と体重の入力
height_cm = float(input('身長(cm)は? '))
weight = float(input('体重(kg)は? '))

# BMIの計算
height = height_cm / 100.0
bmi = weight / (height ** 2)

# 肥満度判定表 - 判定用辞書のリスト
bmi_list = [
    {"min": 0, "max": 18.5, "label": "低体重"},
    {"min": 18.5, "max": 25, "label": "普通体重"},
    {"min": 25, "max": 30, "label": "肥満1度"},
    {"min": 30, "max": 35, "label": "肥満2度"},
    {"min": 35, "max": 40, "label": "肥満3度"},
    {"min": 40, "max": 99, "label": "肥満4度"}]

# 判定
result = "不明"
for range in bmi_list:
    if range["min"] <= bmi < range["max"]:
        result = range["label"]

# 結果表示
print("BMI={:.1f},判定={}".format(bmi, result))

