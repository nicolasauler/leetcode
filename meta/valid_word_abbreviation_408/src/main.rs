fn valid_word_abbreviation(word: String, abbr: String) -> bool {
    let mut w_count = 0;
    let mut a_count = 0;

    while a_count < abbr.len() && w_count < word.len() {
        let w_char = word.chars().nth(w_count).unwrap();
        let a_char = abbr.chars().nth(a_count).unwrap();

        if a_char.is_ascii_alphabetic() && a_char == w_char {
            a_count += 1;
            w_count += 1;
            continue;
        }

        if a_char.is_numeric() {
            let mut number = String::new();
            let mut test_char = Some(a_char);
            while test_char.is_some() && test_char.unwrap().is_numeric() {
                number.push(test_char.unwrap());
                a_count += 1;
                test_char = abbr.chars().nth(a_count);
            }
            if number.chars().nth(0).unwrap() == '0' {
                return false;
            }
            let number: usize = number.parse().unwrap();
            w_count += number;

            if word.chars().nth(w_count) != abbr.chars().nth(a_count) {
                return false;
            }
            continue;
        }

        return false;
    }

    if w_count > word.len() || a_count < abbr.len() {
        return false;
    }
    if w_count < word.len() && a_count >= abbr.len() {
        return false;
    }
    return true;
}

fn valid_word_abbreviation_better_solution(word: String, abbr: String) -> bool {
    let mut w_pos = 0;
    let mut number = 0;

    for a_char in abbr.chars() {
        if a_char.is_ascii_alphabetic() {
            w_pos += number;
            number = 0;
            if w_pos >= word.len() || a_char != word.chars().nth(w_pos).unwrap() {
                return false;
            }
            w_pos += 1;
        } else if a_char.is_numeric() {
            if a_char == '0' && number == 0 {
                return false;
            }
            number = (number * 10) + a_char.to_digit(10).unwrap() as usize
        } else {
            return false;
        }
    }

    return w_pos + number == word.len();
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accept_0() {
        let word = "internationalization";
        let abbr = "i12iz4n";
        let is_valid = valid_word_abbreviation(word.to_owned(), abbr.to_owned());
        assert!(is_valid);
    }

    #[test]
    fn accept_1() {
        let word = "internationalization";
        let abbr = "i5a11o1";
        let is_valid = valid_word_abbreviation(word.to_owned(), abbr.to_owned());
        assert!(is_valid);
    }

    #[test]
    fn fail_0() {
        let word = "hi";
        let abbr = "hi1";
        let is_valid = valid_word_abbreviation(word.to_owned(), abbr.to_owned());
        assert!(!is_valid);
    }

    #[test]
    fn accept_2() {
        let word = "abbreviation";
        let abbr = "a10n";
        let is_valid = valid_word_abbreviation(word.to_owned(), abbr.to_owned());
        assert!(is_valid);
    }

    #[test]
    fn fail_1() {
        let word = "ab";
        let abbr = "a";
        let is_valid = valid_word_abbreviation(word.to_owned(), abbr.to_owned());
        assert!(!is_valid);
    }

    #[test]
    fn accept_better_0() {
        let word = "internationalization";
        let abbr = "i12iz4n";
        let is_valid = valid_word_abbreviation_better_solution(word.to_owned(), abbr.to_owned());
        assert!(is_valid);
    }

    #[test]
    fn accept_better_1() {
        let word = "internationalization";
        let abbr = "i5a11o1";
        let is_valid = valid_word_abbreviation_better_solution(word.to_owned(), abbr.to_owned());
        assert!(is_valid);
    }

    #[test]
    fn fail_better_0() {
        let word = "hi";
        let abbr = "hi1";
        let is_valid = valid_word_abbreviation_better_solution(word.to_owned(), abbr.to_owned());
        assert!(!is_valid);
    }

    #[test]
    fn accept_better_2() {
        let word = "abbreviation";
        let abbr = "a10n";
        let is_valid = valid_word_abbreviation_better_solution(word.to_owned(), abbr.to_owned());
        assert!(is_valid);
    }

    #[test]
    fn fail_better_1() {
        let word = "ab";
        let abbr = "a";
        let is_valid = valid_word_abbreviation_better_solution(word.to_owned(), abbr.to_owned());
        assert!(!is_valid);
    }
}
