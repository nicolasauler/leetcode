fn calculate(s: String) -> i32 {
    let s = s.trim();
    let mut op = '+';
    let mut num: Option<i32> = None;
    let mut curr = 0;
    let mut result = 0;

    // added + to perform last operation
    for c in s.chars().chain("#".chars()) {
        if c.is_numeric() {
            let c_val = c.to_digit(10).unwrap() as i32;
            num = num.map(|n| n * 10 + c_val).or(Some(c_val));
        } else if c != ' ' {
            match op {
                '+' => curr += num.unwrap(),
                '-' => curr -= num.unwrap(),
                '*' => curr *= num.unwrap(),
                '/' => curr /= num.unwrap(),
                _ => (),
            }
            num = None;
            op = c;
        }

        if c == '+' || c == '-' || c == '#' {
            result += curr;
            curr = 0;
        }
    }

    result
}

fn main() {
    let s = "3+2*2".to_string();
    let result = calculate(s);
    println!("Result: {}", result);
}
