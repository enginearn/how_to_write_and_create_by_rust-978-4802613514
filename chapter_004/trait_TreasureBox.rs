trait TreasureBox {
    fn open(&self, key_no: i32) -> bool;
    fn check(&self);
}

struct JewelryBox {
    price: i32,
    key_no: i32,
}

impl TreasureBox for JewelryBox {
    fn open(&self, key_no: i32) -> bool {
        self.key_no == key_no
    }

    fn check(&self) {
        println!("宝箱を開けた。{}G手に入れた!!", self.price);
    }
}

struct TrapBox {
    damage: i32,
}

impl TreasureBox for TrapBox {
    fn open(&self, _key_no: i32) -> bool {
        return true;
    }

    fn check(&self) {
        println!("宝箱は罠だった! {}のダメージを受けた!!", self.damage);
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if !tbox.open(key_no) {
        println!("開けることができなかった!!");
        return;
    }
    tbox.check();
}

fn main() {
    let box1 = JewelryBox {price: 30, key_no: 1};
    let box2 = TrapBox {damage: 10};
    let box3 = JewelryBox {price: 50, key_no: 2};

    let my_key = 2;
    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}
