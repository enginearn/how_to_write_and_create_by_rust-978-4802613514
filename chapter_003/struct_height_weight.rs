struct Body {
    height: f32,
    weight: f32,
}

fn calc_bmi(body: &Body) -> f32 {

    body.weight / (body.height / 100.0).powf(2.0)
}

fn main() {
    let body_1 = Body {
        height: 170.4,
        weight: 70.0,
    };
    println!("bmi: {:.2}", calc_bmi(&body_1));

    let body_2 = Body {
        height: 165.6,
        weight: 60.0,
    };
    println!("bmi: {:.2}", calc_bmi(&body_2));
}
