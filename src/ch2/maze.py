# Pythonで迷路生成
import random

# 迷路の幅を指定
MAP_N = 25 
# 迷路を初期化
maze = []
for y in range(0, MAP_N):
    maze.append([0 for x in range(0, MAP_N)])

# 外周を壁にする
for n in range(0, MAP_N):
    maze[n][0] = maze[n][MAP_N-1] = 1
    maze[0][n] = maze[MAP_N-1][n] = 1

# 2マスに1つ壁を配置
for y in range(2, MAP_N-2):
    for x in range(2, MAP_N-2):
        if x % 2 == 1 or y % 2 == 1: continue
        maze[y][x] = 1
        # 上下左右のいずれかを壁にする
        r = random.randint(0, 3)
        if r == 0: maze[y-1][x] = 1 # 上
        if r == 1: maze[y+1][x] = 1 # 下
        if r == 2: maze[y][x-1] = 1 # 左
        if r == 3: maze[y][x+1] = 1 # 右

# 迷路を画面に出力
tiles = ["  ", "ZZ"]
for y in range(0, MAP_N):
    for x in range(0, MAP_N):
        print(tiles[maze[y][x]], end="")
    print("")

