use std::collections::HashMap;

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut test: HashMap<char, u32> = HashMap::new();

    for char in s.chars() {
        test.entry(char).and_modify(|v| *v += 1).or_insert(1);
    }

    for char in t.chars() {
        if let Some(times) = test.get_mut(&char) {
            if *times > 0 {
                *times -= 1;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn main() {
    let s = "anagram";
    let t = "nagaram";
    let result = is_anagram(s.to_owned(), t.to_owned());
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true() {
        let s = "anagram";
        let t = "nagaram";
        let result = is_anagram(s.to_owned(), t.to_owned());
        assert!(result);
    }

    #[test]
    fn is_false() {
        let s = "rat";
        let t = "car";
        let result = is_anagram(s.to_owned(), t.to_owned());
        assert!(!result);
    }
}
