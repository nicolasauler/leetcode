use std::collections::VecDeque;

fn min_remove_to_make_valid(s: String) -> String {
    let mut balance = 0;
    let mut one_pass_s = String::with_capacity(s.len());

    for char in s.chars() {
        if char == '(' {
            balance += 1;
        }
        if char == ')' {
            if balance <= 0 {
                balance = 0;
                continue;
            }
            balance -= 1;
        }

        one_pass_s.push(char);
    }

    let mut two_pass_s = String::with_capacity(one_pass_s.len());
    for char in one_pass_s.chars().rev() {
        if char == '(' && balance > 0 {
            balance -= 1;
            continue;
        }
        two_pass_s.push(char);
    }

    return two_pass_s.chars().rev().collect();
}

fn min_remove_to_make_valid_better(s: String) -> String {
    let mut balance = 0;
    let mut n_left_par = 0;
    let mut one_pass_s = String::with_capacity(s.len());

    for char in s.chars() {
        if char == '(' {
            balance += 1;
            n_left_par += 1;
        }
        if char == ')' {
            if balance <= 0 {
                balance = 0;
                continue;
            }
            balance -= 1;
        }

        one_pass_s.push(char);
    }

    let mut two_pass_s = String::with_capacity(one_pass_s.len());
    let mut n_good_par = n_left_par - balance;
    for char in one_pass_s.chars() {
        if char == '(' {
            if n_good_par <= 0 {
                continue;
            }
            n_good_par -= 1;
        }
        two_pass_s.push(char);
    }

    return two_pass_s;
}

fn min_remove_to_make_valid_even_better(s: String) -> String {
    let mut stack_of_parentheses: VecDeque<usize> = VecDeque::new();
    let mut s_parsed = String::with_capacity(s.len());
    let mut s_parsed_len = 0;

    for char in s.chars() {
        if char == '(' {
            stack_of_parentheses.push_back(s_parsed_len);
        }
        if char == ')' {
            if stack_of_parentheses.len() == 0 {
                continue;
            }
            stack_of_parentheses.pop_front();
        }
        s_parsed.push(char);
        s_parsed_len += 1;
    }

    while let Some(pos) = stack_of_parentheses.pop_back() {
        s_parsed.replace_range(pos..pos + 1, "");
    }

    return s_parsed;
}

fn main() {
    //    let s = "lee(t(c)o)de)";
    //    let s_parsed = min_remove_to_make_valid(s.to_owned());
    //    println!("{s_parsed}");
    //
    //    let s = "a)b(c)d";
    //    let s_parsed = min_remove_to_make_valid(s.to_owned());
    //    println!("{s_parsed}");

    //let s = "(a(b(c)d)";
    //let s_parsed = min_remove_to_make_valid_better(s.to_owned());
    //println!("{s_parsed}");

    let s = "))((";
    let s_parsed = min_remove_to_make_valid_even_better(s.to_owned());
    println!("{s_parsed}");

    let s = "lee(t(c)o)de)";
    let s_parsed = min_remove_to_make_valid_even_better(s.to_owned());
    println!("{s_parsed}");

    let s = "a)b(c)d";
    let s_parsed = min_remove_to_make_valid_even_better(s.to_owned());
    println!("{s_parsed}");

    let s = "(a(b(c)d)";
    let s_parsed = min_remove_to_make_valid_even_better(s.to_owned());
    println!("{s_parsed}");
}
