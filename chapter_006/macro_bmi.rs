macro_rules! bmi_select {
    ($bmi: expr; $($label: expr => $range: expr),*) => {{
        let mut result = String::new();
        $(if $bmi >= $range.0 && $bmi < $range.1 {
            result = $label.to_string();
        })*
        result
    }};
}

fn main() {
    let h: f32 = 175.0;
    let w: f32 = 70.0;
    let bmi = w / (h / 100.00).powf(2.0);

    let label = bmi_select!(bmi;
        "低体重" => (0.0, 18.5),
        "普通" => (18.5, 24.0),
        "肥満度1" => (24.0, 28.0),
        "肥満" => (28.0, 100.0)
    );
    println!("bmi: {}, label: {}", format!("{:.2}", bmi), label);
}
