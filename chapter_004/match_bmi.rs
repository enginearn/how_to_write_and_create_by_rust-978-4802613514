fn print_bmi(height: f32, weight: Option<f32>) {
    let bmi: Option<f32> = match weight {
        Some(w) => Some(w / (height /100.0).powf(2.0)),
        None => None,
    };

    let msg = match bmi {
        Some(n) if n < 18.5 => "Underweight",
        Some(n) if n < 25.0 => "Normal",
        Some(n) if n < 30.0 => "Overweight",
        Some(n) if n < 35.0 => "Obese",
        Some(n) if n < 40.0 => "Extremely obese",
        Some(_) => "Morbidly obese",
        None => "No BMI",
    };

    println!("BMI is {:.2} and you are {}", bmi.unwrap_or(0.0), msg);
}

fn main() {
    let height = 170.0;
    print_bmi(height, Some(53.5));
    print_bmi(height, Some(60.0));
    print_bmi(height, None);
}
