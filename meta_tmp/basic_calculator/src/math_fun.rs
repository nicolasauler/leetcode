#![allow(dead_code)]

fn polish_calculator(polish_notation_exp: String) -> i32 {
    let op_vec: Vec<&str> = polish_notation_exp.trim().split_whitespace().collect();
    let mut stack: Vec<String> = Vec::with_capacity(op_vec.len());

    for strin in op_vec {
        println!("{stack:?}");
        if strin == "+" {
            let right_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let left_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let val = left_operand + right_operand;
            let val_str = val.to_string();
            stack.push(val_str);
        } else if strin == "-" {
            let right_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let left_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let val = left_operand - right_operand;
            let val_str = val.to_string();
            stack.push(val_str);
        } else if strin == "*" {
            let right_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let left_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let val = left_operand * right_operand;
            let val_str = val.to_string();
            stack.push(val_str);
        } else if strin == "/" {
            let right_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let left_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let val = left_operand / right_operand;
            let val_str = val.to_string();
            stack.push(val_str);
        } else {
            stack.push(strin.to_owned());
        }
    }

    stack.pop().unwrap().parse::<i32>().unwrap()
}

fn calculate_from_polish(rpn: &[String]) -> i32 {
    let mut stack: Vec<String> = Vec::with_capacity(rpn.len());

    for strin in rpn {
        if strin == "+" {
            let right_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let left_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let val = left_operand + right_operand;
            let val_str = val.to_string();
            stack.push(val_str);
        } else if strin == "-" {
            let right_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let left_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let val = left_operand - right_operand;
            let val_str = val.to_string();
            stack.push(val_str);
        } else if strin == "*" {
            let right_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let left_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let val = left_operand * right_operand;
            let val_str = val.to_string();
            stack.push(val_str);
        } else if strin == "/" {
            let right_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let left_operand: i32 = stack.pop().unwrap().parse().unwrap();
            let val = left_operand / right_operand;
            let val_str = val.to_string();
            stack.push(val_str);
        } else {
            stack.push(strin.to_owned());
        }
    }

    stack.pop().unwrap().parse::<i32>().unwrap()
}

/// Shunting yard algorithm
fn infix_to_postfix(infix_string: String) -> Vec<String> {
    let mut postfix: Vec<String> = Vec::new();
    let mut operator_stack: Vec<&str> = Vec::new();
    for token in infix_string.trim().split_whitespace() {
        if token == "+" || token == "-" {
            while operator_stack.last().is_some_and(|&x| x != "(") {
                postfix.push(operator_stack.pop().unwrap().to_owned());
            }
            operator_stack.push(token);
        } else if token == "*" || token == "/" {
            operator_stack.push(token);
        } else if token == "(" {
            operator_stack.push(token);
        } else if token == ")" {
            while operator_stack.last().is_some_and(|&x| x != "(") {
                postfix.push(operator_stack.pop().unwrap().to_owned());
            }
            operator_stack.pop();
        } else {
            postfix.push(token.to_owned());
        }
    }

    while let Some(stacked_token) = operator_stack.pop() {
        postfix.push(stacked_token.to_owned());
    }
    println!("{postfix:?}");

    postfix
}

pub fn basic_calculator(expression: String) -> i32 {
    let postfix = infix_to_postfix(expression);
    println!("{:?}", postfix);
    calculate_from_polish(&postfix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let exp = "1 + 1".to_owned();
        let res = basic_calculator(exp);
        assert_eq!(res, 2_i32);
    }

    #[test]
    fn test_sub() {
        let exp = "2 - 1 + 2".to_owned();
        // 2 1 - 2 +
        let res = basic_calculator(exp);
        assert_eq!(res, 3_i32);
    }

    #[test]
    fn test_complex() {
        let exp = "(1 + (4 + 5 + 2) - 3) + (6 + 8)".to_owned();
        // 1 4 5 + 2 + + 3 - 6 8 + +
        // 1 -> 4 -> 5
        // 4 + 5 = 9
        // 1 -> 9 -> 2
        // 9 + 2 = 11
        // 1 11 +
        // 1 + 11 =12
        // 12 -> 3 -> -
        // 12 - 3 = 9
        let res = basic_calculator(exp);
        assert_eq!(res, 3_i32);
    }
}
