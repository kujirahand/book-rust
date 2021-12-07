# Pythonで動的ライブラリを利用する
import platform, os
from ctypes import *

# PythonでOSを判定 --- (*1)
pf = platform.system()
print(pf)

# Windowsの場合 --- (*2)
if pf == 'Windows': libfile = 'mycalc.dll'
# macOSの場合
elif pf == 'Darwin': libfile = 'libmycalc.dylib'
# Linuxの場合
else: libfile = 'libmycalc.so'

# 動的ライブラリのパスを指定 --- (*3)
libpath = os.path.join(os.path.dirname(__file__), libfile)
print("lib=", libpath)

# ライブラリをロード --- (*4)
mycalc = cdll.LoadLibrary(libpath)
# Rustのライブラリを実行 --- (*5)
print(mycalc.rust_mul(100, 8))
print(mycalc.rust_mul(8, 9))

