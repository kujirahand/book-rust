use rand::Rng;

// 迷路の幅を指定 --- (*1)
const MAP_N: usize = 25;

fn main() {
    // 乱数生成器を用意 --- (*2)
    let mut rng = rand::thread_rng();
    // 迷路を初期化 --- (*3)
    let mut maze = [[0; MAP_N]; MAP_N];
    // 外周を壁にする --- (*4)
    for n in 0..MAP_N {
        maze[n][0] = 1;
        maze[0][n] = 1;
        maze[n][MAP_N-1] = 1;
        maze[MAP_N-1][n] = 1;
    }
    // 2マスに1つ壁を配置 --- (*5)
    for y in 2..MAP_N-2 {
        for x in 2..MAP_N-2 {
            if x % 2 == 1 || y % 2 == 1 { continue; }
            maze[y][x] = 1; // 壁に
            // 上下左右のいずれかを壁にする --- (*6)
            let r = rng.gen_range(0..=3);
            match r {
                0 => maze[y-1][x] = 1, // 上
                1 => maze[y+1][x] = 1, // 下
                2 => maze[y][x-1] = 1, // 左
                3 => maze[y][x+1] = 1, // 右
                _ => {},
            }
        }
    }
    // 迷路を画面に出力 --- (*7)
    let tiles = ["  ", "ZZ"];
    for y in 0..MAP_N {
        for x in 0..MAP_N {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}
