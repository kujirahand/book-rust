# ファイル名を指定 --- (*1)
afile = "./fizzbuzz_python.txt"
bfile = "./fizzbuzz_rust.txt"

# ファイルを文字列として読み込む --- (*2)
with open(afile, "r") as fp:
    astr = fp.read()
with open(bfile, "r") as fp:
    bstr = fp.read()

# 念のため余分な空行をトリム
astr = astr.strip()
bstr = bstr.strip()

# 比較 --- (*3)
if astr == bstr:
    print("ok")
else:
    print("ng")

