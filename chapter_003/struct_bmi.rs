struct BmiRange {
    min: f64,
    max: f64,
    lavel: &'static str,
}

fn bmi_calc(height: f64, weight: f64) -> f64 {
    weight / (height / 100.0).powf(2.0)
}

fn user_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<f64>().expect("Failed to parse input")
}

fn main() {
    let height = user_input("Enter your height (cm): ");
    let weight = user_input("Enter your weight (kg): ");
    let bmi = bmi_calc(height, weight);

    let bmi_ranges = vec![
    BmiRange {
        min: 18.5,
        max: 25.0,
        lavel: "Normal",
    },
    BmiRange {
        min: 25.0,
        max: 30.0,
        lavel: "Overweight",
    },
    BmiRange {
        min: 30.0,
        max: 35.0,
        lavel: "Obesity",
    },
    BmiRange {
        min: 35.0,
        max: 40.0,
        lavel: "Severe obesity",
    },
];

    println!("Your BMI is {:.2}", bmi);
    for range in bmi_ranges {
        if bmi >= range.min && bmi <= range.max {
            println!("You are in the {} range.", range.lavel);
        }
    }
}
