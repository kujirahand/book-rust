// 宝箱の振る舞いを定義するトレイト --- (*1)
trait TreasureBox {
    fn open(&self, key_no: i32) -> bool;
    fn check(&self);
}

// 宝石箱を表現する構造体を定義 --- (*2)
struct JewelryBox {
    price: i32, // 金貨何枚分か
    key_no: i32, // 何番の鍵があれば開くか
}
impl TreasureBox for JewelryBox {
    fn open(&self, key_no: i32) -> bool {
        // 指定の鍵でのみ箱は空く
        self.key_no == key_no
    }
    fn check(&self) {
        println!("宝石箱だった！金貨{}枚入手。", self.price);
    }
}

// 罠の箱を表現する構造体を定義 --- (*3)
struct TrapBox {
    damage: i32,
}
impl TreasureBox for TrapBox {
    fn open(&self, _key: i32) -> bool {
        return true; // どんな鍵でも開く
    }
    fn check(&self) {
        println!("罠だった。{}のダメージ。", self.damage);
    }
}

// 冒険者が箱を開ける動作 --- (*4)
fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("鍵が合わず宝箱が空きません。");
        return;
    }
    tbox.check();
}

fn main() {
    // いろいろな宝箱を準備 --- (*5)
    let box1 = JewelryBox { price: 30, key_no: 1 };
    let box2 = TrapBox { damage: 3 };
    let box3 = JewelryBox { price: 20, key_no: 2 };
    // 冒険者が宝箱を手持ちの鍵で開ける --- (*6)
    let my_key = 2;
    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}


