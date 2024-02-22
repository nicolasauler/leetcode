fn valid_palindrome_i(s: String) -> bool {
    let middle = s.len() / 2;

    let (left, right) = s.split_at(middle);
    for (l_char, r_char) in left.chars().zip(right.chars().rev()) {
        if l_char != r_char {
            return false;
        }
    }

    return true;
}

fn valid_palindrome_with_atmost_one_deletion(s: String) -> bool {
    let middle = s.len() / 2;
    let (left, right) = s.split_at(middle);

    let mut l_pos = 0;
    let mut r_pos = 0;

    while l_pos < left.len() {
        let l_char = left.chars().nth(l_pos);
        let r_char = right.chars().rev().nth(r_pos);

        if (r_char.is_none() && l_pos == left.len() - 1)
            || (l_char.is_none() && r_pos == right.len() - 1)
        {
            break;
        }

        if l_char == r_char {
            l_pos += 1;
            r_pos += 1;
        } else {
            if left.chars().nth(l_pos + 1) == right.chars().rev().nth(r_pos) {
                l_pos += 1;
            } else if right.chars().rev().nth(r_pos + 1) == left.chars().nth(l_pos) {
                r_pos += 1;
            }
        }

        if r_pos > l_pos {
            if r_pos - l_pos > 1 {
                return false;
            }
        } else {
            if l_pos - r_pos > 1 {
                return false;
            }
        }
    }

    return true;
}

fn main() {
    let s = "enbbe";
    let is_valid = valid_palindrome_with_atmost_one_deletion(s.to_owned());
    println!("{}", is_valid);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_palindrome_with_atmost_one_deletion_0() {
        let s = "aba";
        let is_valid = valid_palindrome_with_atmost_one_deletion(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_with_atmost_one_deletion_1() {
        let s = "abca";
        let is_valid = valid_palindrome_with_atmost_one_deletion(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_with_atmost_one_deletion_4() {
        let s = "abbbca";
        let is_valid = valid_palindrome_with_atmost_one_deletion(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_with_atmost_one_deletion_2() {
        let s = "abba";
        let is_valid = valid_palindrome_with_atmost_one_deletion(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_with_atmost_one_deletion_3() {
        let s = "hannah";
        let is_valid = valid_palindrome_with_atmost_one_deletion(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_i() {
        let s = "aba";
        let is_valid = valid_palindrome_i(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_i_2() {
        let s = "abca";
        let is_valid = valid_palindrome_i(s.to_owned());
        assert_eq!(is_valid, false);
    }

    #[test]
    fn test_valid_palindrome_i_3() {
        let s = "abba";
        let is_valid = valid_palindrome_i(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_i_4() {
        let s = "hannah";
        let is_valid = valid_palindrome_i(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_i_5() {
        let s = "aaa";
        let is_valid = valid_palindrome_i(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_i_6() {
        let s = "aaaa";
        let is_valid = valid_palindrome_i(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_i_7() {
        let s = "abcba";
        let is_valid = valid_palindrome_i(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_i_8() {
        let s = "";
        let is_valid = valid_palindrome_i(s.to_owned());
        assert_eq!(is_valid, true);
    }

    #[test]
    fn test_valid_palindrome_i_9() {
        let s = "abcca";
        let is_valid = valid_palindrome_i(s.to_owned());
        assert_eq!(is_valid, false);
    }
}
