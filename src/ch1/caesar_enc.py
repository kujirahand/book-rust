# Pythonでシーザー暗号に変換する関数
def encrypt(text, shift):
    # 'A'と'Z'の文字コードを得る --- (*1)
    code_a = ord('A')
    code_z = ord('Z')
    # 結果を代入する変数を用意
    result = ""
    # 一文字ずつ繰り返す --- (*2)
    for ch in text:
        # 文字コードに変換
        code = ord(ch)
        # AからZの間か？ --- (*3)
        if code_a <= code <= code_z:
            # shift分だけ並びをずらす --- (*4)
            code = (code - code_a + shift) % 26 + code_a
        # 文字コードから文字に変換 --- (*5)
        result += chr(code)
    return result

# 関数を呼び出す
enc = encrypt("I LOVE YOU.", 3)
dec = encrypt(enc, -3)
print(enc, "=>", dec)

