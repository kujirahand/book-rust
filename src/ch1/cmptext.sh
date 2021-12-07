#!/bin/sh
# 結果をテキストファイルに書き出す
python3 fizzbuzz.py > fb_python.txt
rustc fizzbuzz.rs && ./fizzbuzz > fb_rust.txt
# diffコマンドでテキストを比較
diff fb_python.txt fb_rust.txt

