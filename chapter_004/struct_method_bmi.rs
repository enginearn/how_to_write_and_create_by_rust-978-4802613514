struct Body {
    height: f32,
    weight: f32,
}

impl Body {
    fn bmi(&self) -> f32 {
        self.weight / ((self.height / 100.0).powi(2))
    }

    fn calc_per(&self) -> f32 {
        self.bmi() / 22.0 * 100.0
    }
}

fn main() {
    let john = Body {
        height: 175.0,
        weight: 80.0,
    };
    println!("BMI: {:.2}", john.bmi());
    println!("乖離率: {:.2}", john.calc_per());
}
