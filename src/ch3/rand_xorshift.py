# Pythonで疑似乱数を生成する
import time

# 乱数生成のための種をグローバルで指定 --- (*1)
SEED = 0

# startからendの乱数を生成 --- (*2)
def rand(start, end):
    global SEED
    # 初回のみ乱数を初期化
    if SEED == 0:
        # 現在時刻で種を初期化
        SEED = int(time.time() * 1000)
    # 乱数を生成
    SEED ^= (SEED << 13) & 0xFFFFFFFF
    SEED ^= (SEED >> 17) & 0xFFFFFFFF
    SEED ^= (SEED << 5) & 0xFFFFFFFF
    return SEED % (end - start + 1) + start

# 100回乱数を表示 --- (*3)
for _ in range(100):
    print(rand(1, 6))

