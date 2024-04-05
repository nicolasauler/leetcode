use std::collections::{HashMap, HashSet};

fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut test: HashMap<char, Vec<usize>> = HashMap::new();
    for (pos, char) in s.chars().enumerate() {
        test.entry(char)
            .and_modify(|v| v.push(pos))
            .or_insert(vec![pos]);
    }

    let mut set: HashSet<char> = HashSet::new();
    for char in p.chars() {
        set.insert(char);
    }

    let mut count = 0;
    let mut occurences: Vec<i32> = Vec::new();
    let bck_set = set.clone();

    for (i, char) in s.chars().enumerate() {
        if set.remove(&char) {
            count += 1;
        } else {
            set = bck_set.clone();
            count = 0;
            continue;
        }

        if count == p.len() - 1 {
            occurences.push(i as i32 - count as i32);
        }
    }

    occurences
}

fn main() {
    let s = "cbaebabacd".to_owned();
    let p = "abc".to_owned();
    let occurences = find_anagrams(s, p);
    println!("{:?}", occurences);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let s = "cbaebabacd".to_owned();
        let p = "abc".to_owned();
        let occurences = find_anagrams(s, p);
        assert_eq!(occurences, vec![0, 6]);
    }

    #[test]
    fn test_1() {
        let s = "abab".to_owned();
        let p = "ab".to_owned();
        let occurences = find_anagrams(s, p);
        assert_eq!(occurences, vec![0, 1, 2]);
    }
}
