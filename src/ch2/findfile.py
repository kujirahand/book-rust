import sys, os

# コマンドライン引数を確認
if len(sys.argv) < 3:
    print("findfile.py (path) (keyword)")
    quit()
# コマンドライン引数を得る
target_dir = sys.argv[1]
keyword = sys.argv[2]

# 指定のディレクトリを検索
for dirname, dirs, files in os.walk(target_dir):
    for file in files:
        if keyword in file:
            fullpath = os.path.join(dirname, file)
            print(fullpath)

