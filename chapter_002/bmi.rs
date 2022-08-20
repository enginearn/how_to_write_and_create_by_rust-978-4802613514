fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse::<f64>().expect("Failed to parse input")
}

fn main() {
    let height = input("Height: ");
    let weight = input("Weight: ");

    let bmi = weight / ((height/ 100.0).powf(2.0));
    println!("BMI: {:.2}", bmi);

    if bmi < 18.5 {
        println!("Underweight");
    } else if bmi < 25.0 {
        println!("Normal");
    } else if bmi < 30.0 {
        println!("Overweight (1)");
    } else if bmi < 35.0 {
        println!("Overweight (2)");
    } else if bmi < 40.0 {
        println!("Overweight (3)");
    } else {
        println!("Obese");
    }
}
