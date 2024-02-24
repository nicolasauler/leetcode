fn valid_palindrome(s: String) -> bool {
    let s = s.as_bytes();
    let mut len = s.len();

    let mut l = 0;
    let mut r = len - 1;
    while (l < len / 2) && (r >= len / 2) {
        let l_char = s[l];
        let r_char = s[r];

        if !l_char.is_ascii_alphanumeric() {
            l += 1;
            len += 1;
            continue;
        }
        if !r_char.is_ascii_alphanumeric() {
            r -= 1;
            len -= 1;
            continue;
        }

        if l_char.to_ascii_lowercase() != r_char.to_ascii_lowercase() {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}

fn main() {
    let s = "A man, a plan, a canal: Panama";
    let result = valid_palindrome(s.to_string());
    println!("Result: {}", result);

    let s = "race a car";
    let result = valid_palindrome(s.to_string());
    println!("Result: {}", result);

    let s = " ";
    let result = valid_palindrome(s.to_string());
    println!("Result: {}", result);

    let s = "a.b,.";
    let result = valid_palindrome(s.to_string());
    println!("Result: {}", result);
}
