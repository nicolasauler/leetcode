use std::collections::HashMap;

fn majority_element_not_that_good(nums: Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, usize> = HashMap::new();

    for num in nums {
        match counts.get(&num) {
            Some(count) => counts.insert(num, count + 1),
            None => counts.insert(num, 1),
        };
    }

    *counts.iter().max_by_key(|(_k, &v)| v).unwrap().0
}

fn majority_element_voting(nums: Vec<i32>) -> i32 {
    let mut major = nums[0];
    let mut count = 1;

    for i in 1..nums.len() {
        if count == 0 {
            major = nums[i];
            count = 1;
        } else {
            if nums[i] != major {
                count -= 1;
            } else {
                count += 1;
            }
        }
    }

    return major;
}

fn main() {
    let nums = vec![3, 2, 3];
    let result = majority_element_not_that_good(nums);
    println!("Result: {}", result);

    let nums = vec![3, 2, 3];
    let result = majority_element_voting(nums);
    println!("Result: {}", result);

    let nums = vec![10, 9, 9, 9, 10];
    let result = majority_element_voting(nums);
    println!("Result: {}", result);
}
