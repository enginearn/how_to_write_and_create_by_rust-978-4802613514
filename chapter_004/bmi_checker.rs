struct BmiRange {
    min: f32,
    max: f32,
    label: String,
}

impl BmiRange{
    fn new(min: f32, max: f32, label: &str) -> Self {
        BmiRange {
            min,
            max,
            label: label.to_string(),
        }
    }

    fn test(&self, v: f32) -> bool {
        (self.min <= v) && (v < self.max)
    }
}

struct Body {
    height_cm: f32,
    weight_kg: f32,
    name: String,
}

impl Body {
    fn new(height_cm: f32, weight_kg: f32, name: &str) -> Self {
        Body {
            height_cm,
            weight_kg,
            name: name.to_string(),
        }
    }

    fn bmi(&self) -> f32 {
        self.weight_kg / (self.height_cm / 100.0).powi(2)
    }

    fn print_result(&self) {
        let bmi = self.bmi();
        let bmi_list = [
            BmiRange::new(0.0, 18.5, "underweight"),
            BmiRange::new(18.5, 25.0, "normal"),
            BmiRange::new(25.0, 30.0, "overweight Level 1"),
            BmiRange::new(30.0, 35.0, "obese Level 2"),
            BmiRange::new(35.0, 40.0, "severely obese  Level 3"),
            BmiRange::new(40.0, 99.0, "morbidly obese Level 4"),
        ];

        let mut result = String::from("Unknown");
        for range in bmi_list {
            if range.test(bmi) {
                result = range.label.clone();
                break;
            }
        }
        println!("{} (BMI: {:.2}): {}", self.name, self.bmi(), result);
    }
}

fn main() {
    let body = Body::new(164.0, 60.0, "John");
    body.print_result();
    let body = Body::new(165.0, 55.0, "Mary");
    body.print_result();
    let body = Body::new(178.0, 90.0, "Jane");
    body.print_result();
}
