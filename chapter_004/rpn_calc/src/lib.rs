// スラッシュ3つでライブラリドキュメントになる
/// Eval RPN
/// # Example
/// ```
/// let src = String::from("1 2 + 3 4 + *");
/// let ans = eval_rpn::eval(src).unwrap();
/// println!("{}", ans); // 21
/// ```


pub fn eval(src: String) -> Result<f64, String> {
    let mut stack: Vec<f64> = vec![];
    let input = src.split_whitespace();
    for token in input {
        let token = token.trim();
        match token.parse::<f64>() {
            Ok(v) => { stack.push(v); continue; },
            Err(_) => 0.0,
        };

        let b = stack.pop().expect("Stack underflow");
        let a = stack.pop().expect("Stack underflow");
        match token {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "*" => stack.push(a * b),
            "/" => stack.push(a / b),
            "%" => stack.push(a % b),
            "^" => stack.push(a.powf(b)),
            _ => return Err(format!("Unknown operator: {}", token)),
        }
    }
    if stack.len() == 0 {
        return Err(format!("No result..."));
    }
    if stack.len() > 1 {
        return Err(format!("Stack overflow"));
    }

    Ok(stack.pop().unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let src = String::from("1 2 + 3 4 + *");
        let ans = eval(src).unwrap();
        assert_eq!(ans, 21.0);
    }
}
