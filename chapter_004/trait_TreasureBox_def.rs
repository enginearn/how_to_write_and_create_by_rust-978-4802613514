trait TreasureBox {
    fn open(&self, key_no: i32) -> bool {
        self.get_key_no() == key_no
    }
    fn check(&self);
    fn get_key_no(&self) -> i32;
}

struct JewelryBox {
    price: i32,
    key_no: i32,
}

impl TreasureBox for JewelryBox {
    fn check(&self) {
        println!("宝箱を開けた。{}G手に入れた!!", self.price);
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

struct EmptyBox {
    key_no: i32,
}

impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("宝箱は空だった!!");
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
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

    fn get_key_no(&self) -> i32 {
        return 0;
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
    let box3 = EmptyBox {key_no: 2};
    let box4 = JewelryBox {price: 50, key_no: 2};

    let my_key = 2;
    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
    open_box(&box4, my_key);
}
