use std::{cmp::Reverse, collections::BinaryHeap};

fn h_index(citations: Vec<i32>) -> i32 {
    let mut curr = 1;
    let mut previous: BinaryHeap<Reverse<i32>> = BinaryHeap::new();

    previous.push(Reverse(*citations.iter().next().unwrap()));
    for citation in citations {
        if citation >= curr && *previous.peek().unwrap() >= Reverse(curr) {
            curr += 1;
            previous.push(Reverse(citation));
        }
    }

    curr
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leetcode_test1() {
        let citations: Vec<i32> = vec![3, 0, 6, 1, 5];
        let result = h_index(citations);
        assert_eq!(result, 3);
    }

    #[test]
    fn leetcode_test2() {
        let citations: Vec<i32> = vec![1, 3, 1];
        let result = h_index(citations);
        assert_eq!(result, 1);
    }
}

fn main() {
    let citations: Vec<i32> = vec![3, 0, 6, 1, 5];
    let result = h_index(citations);
    println!("{result}");
}
