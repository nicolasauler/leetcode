#[derive(Debug)]
enum TypeToken {
    Operand(i32),
    Operator(char),
}

enum Operations {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operations {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        match self {
            Operations::Add => a + b,
            Operations::Sub => a - b,
            Operations::Mul => a * b,
            Operations::Div => a / b,
        }
    }

    fn from_char(c: char) -> Option<Operations> {
        match c {
            '+' => Some(Operations::Add),
            '-' => Some(Operations::Sub),
            '*' => Some(Operations::Mul),
            '/' => Some(Operations::Div),
            _ => None,
        }
    }
}

fn infix_to_postfix(s: &str) -> Vec<TypeToken> {
    let mut number: Option<i32> = None;
    let s = s.trim();
    let mut postfix: Vec<TypeToken> = Vec::with_capacity(s.len());
    let mut operator_stack: Vec<char> = Vec::new();

    for c in s.chars() {
        if c >= '0' && c <= '9' {
            let c_val = c.to_digit(10).unwrap() as i32;
            number = number.map(|n| n * 10 + c_val).or(Some(c_val));
        } else if c == '+' || c == '-' || c == '*' || c == '/' || c == '(' || c == ')' {
            if number.is_some() {
                postfix.push(TypeToken::Operand(number.unwrap()));
                number = None;
            }
            if c == '+' || c == '-' {
                while operator_stack.last().is_some_and(|op| *op != '(') {
                    postfix.push(TypeToken::Operator(operator_stack.pop().unwrap()));
                }
            } else if c == '*' || c == '/' {
                while operator_stack
                    .last()
                    .is_some_and(|op| *op == '*' || *op == '/')
                {
                    postfix.push(TypeToken::Operator(operator_stack.pop().unwrap()));
                }
            } else if c == ')' {
                while operator_stack.last().is_some_and(|op| *op != '(') {
                    postfix.push(TypeToken::Operator(operator_stack.pop().unwrap()));
                }
                postfix.pop();
                continue;
            }
            operator_stack.push(c);
        } else {
            continue;
        }
    }
    if number.is_some() {
        postfix.push(TypeToken::Operand(number.unwrap()));
    }
    while let Some(stacked_token) = operator_stack.pop() {
        postfix.push(TypeToken::Operator(stacked_token));
    }

    postfix
}

fn calculate_from_postfix(rpn: &[TypeToken]) -> i32 {
    let mut operand_stack: Vec<i32> = Vec::new();

    for token in rpn {
        match token {
            TypeToken::Operand(n) => operand_stack.push(*n),
            TypeToken::Operator(op) => {
                let b = operand_stack.pop().unwrap();
                let a = operand_stack.pop().unwrap();
                let operation = Operations::from_char(*op).unwrap();
                operand_stack.push(operation.calculate(a, b));
            }
        }
    }

    operand_stack.pop().unwrap()
}

pub fn calculate(s: String) -> i32 {
    let postfix = infix_to_postfix(&s);
    println!("{:?}", postfix);
    calculate_from_postfix(&postfix)
}
