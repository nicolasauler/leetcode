fn valid_parentheses(phrase: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for char in phrase.chars() {
        if char == '(' || char == '{' || char == '[' {
            stack.push(char);
        } else {
            match stack.pop() {
                Some(c) => match c {
                    '(' => {
                        if char != ')' {
                            return false;
                        }
                    }
                    '{' => {
                        if char != '}' {
                            return false;
                        }
                    }
                    '[' => {
                        if char != ']' {
                            return false;
                        }
                    }
                    _ => return false,
                },
                None => return false,
            }
        }
    }

    if stack.len() > 0 {
        return false;
    }
    true
}

fn main() {
    println!("Hello, world!");
}
