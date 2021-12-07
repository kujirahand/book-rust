rem === Windowsの場合 ===
rem  結果をテキストファイルに書き出す
python3 fizzbuzz.py > fb_python.txt
rustc fizzbuzz.rs && ./fizzbuzz > fb_rust.txt
rem diffコマンドでテキストを比較
fc fb_python.txt fb_rust.txt
