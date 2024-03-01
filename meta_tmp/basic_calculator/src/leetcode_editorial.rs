pub fn calculate(s: String) -> i32 {
    let mut result = 0;
    let mut current = 0;
    let mut num = 0;
    let mut op = '+';
    let s = s.replace(" ", "");
    for i in (s + "+").chars() {
        if i.is_digit(10) {
            num = num * 10 + i.to_digit(10).unwrap() as i32;
        } else {
            match op {
                '+' => current += num,
                '-' => current -= num,
                '*' => current *= num,
                '/' => current /= num,
                _ => (),
            }
            num = 0;
            op = i;
        }
        if i == '+' || i == '-' {
            result += current;
            current = 0;
        }
    }
    result
}
