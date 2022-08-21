struct CarSpec {
    model: u32,
    name: String,
    color: u32,
    price: u32,
    cc: u32,
    speed: u32,
}

fn main() {
    let car_1 = CarSpec {
        model: 2017,
        name: "BMW".to_string(),
        color: 0xFF0000,
        price: 100000,
        cc: 1200,
        speed: 200,
    };
    println!("model: {model}, name: {name}, color: {color:06x}, price: {price}, {cc}cc, {speed}km/h",
                model=car_1.model, name=car_1.name, color=car_1.color, price=car_1.price, cc=car_1.cc, speed=car_1.speed);

    let car_2 = CarSpec {
        model: 2018,
        name: "Audi".to_string(),
        color: 0x00FF00,
        price: 200000,
        cc: 1400,
        speed: 250,
    };
    println!("model: {model}, name: {name}, color: {color:06x}, price: {price}, {cc}cc, {speed}km/h",
                model=car_2.model, name=car_2.name, color=car_2.color, price=car_2.price, cc=car_2.cc, speed=car_2.speed);
}
