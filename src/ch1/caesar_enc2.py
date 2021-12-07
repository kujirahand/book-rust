# Pythonでシーザー暗号に変換する関数(その2)
def encrypt(text, shift):
    a = ord('A')
    conv = lambda n: chr((ord(n) - a + shift) % 26 + a)
    enc1 = lambda n: conv(n) if 'A' <= n <= 'Z' else n
    return ''.join([enc1(n) for n in text])

# 関数を実行
enc = encrypt("I LOVE YOU.", 3)
dec = encrypt(enc, -3)
print(enc, "=>", dec)

