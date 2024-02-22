#![allow(dead_code)]

fn valid_palindrome_i(s: String) -> bool {
    let middle = s.len() / 2;

    let (left, right) = s.split_at(middle);
    for (l_char, r_char) in left.chars().zip(right.chars().rev()) {
        if l_char != r_char {
            return false;
        }
    }

    true
}

fn valid_palindrome_with_deletions(s: String, n_deletions: usize) -> bool {
    let middle = s.len() / 2;
    let (left, right) = s.split_at(middle);

    let mut l_pos = 0;
    let mut r_pos = 0;

    while l_pos < left.len() {
        let l_char = left.chars().nth(l_pos);
        let r_char = right.chars().rev().nth(r_pos);

        if l_char != r_char {
            if n_deletions == 0 {
                return false;
            }

            let de_reverted_r_pos = right.len() - r_pos;
            if valid_palindrome_with_deletions(
                left[l_pos + 1..].to_owned() + &right[..de_reverted_r_pos],
                n_deletions - 1,
            ) {
                return true;
            }

            if valid_palindrome_with_deletions(
                left[l_pos..].to_owned() + &right[..de_reverted_r_pos - 1],
                n_deletions - 1,
            ) {
                return true;
            }

            return false;
        }

        l_pos += 1;
        r_pos += 1;
    }

    true
}

fn check_simple_palindrome(s: &[u8]) -> bool {
    let len = s.len();
    for i in 0..(len / 2) {
        if s[i] != s[len - i - 1] {
            return false;
        }
    }
    true
}

/// approach using two pointers and when we know only english characters
fn valid_palindrome_better_for_leetcode(s: String) -> bool {
    let s = s.as_bytes();
    let len = s.len();

    for l in 0..(len / 2) {
        if s[l] != s[len - l - 1] {
            let l_advance = &s[l + 1..len - l];
            let r_retreat = &s[l..len - l - 1];
            return check_simple_palindrome(l_advance) || check_simple_palindrome(r_retreat);
        }
    }

    true
}

fn valid_palindrome_ext_n_deletions_leet(s: String, n_deletions: usize) -> bool {
    let s = s.as_bytes();
    valid_palindrome_ext_n_deletions(s, n_deletions)
}

fn valid_palindrome_ext_n_deletions(s: &[u8], n_deletions: usize) -> bool {
    let len = s.len();

    for l in 0..(len / 2) {
        if s[l] != s[len - l - 1] {
            if n_deletions == 0 {
                return false;
            }

            if valid_palindrome_ext_n_deletions(&s[l..len - l - 1], n_deletions - 1)
                || valid_palindrome_ext_n_deletions(&s[l + 1..len - l], n_deletions - 1)
            {
                return true;
            }

            return false;
        }
    }

    true
}

fn main() {
    let s = "abcdeca";
    let is_valid = valid_palindrome_ext_n_deletions_leet(s.to_string(), 2);
    println!("Is valid: {}", is_valid);

    let s = "abbababa";
    let is_valid = valid_palindrome_ext_n_deletions_leet(s.to_string(), 1);
    println!("Is valid: {}", is_valid);
}
