fn remove_duplicates_bad(nums: &mut Vec<i32>) -> i32 {
    let len = nums.len();
    let mut nums2: Vec<i32> = vec![0; len];
    nums2[0] = nums[0];
    let mut count = 1;

    for i in 0..(len - 1) {
        if nums[i] != nums[i + 1] {
            nums2[count] = nums[i + 1];
            count += 1;
        }
    }

    nums.copy_from_slice(&nums2);
    count as i32
}

fn remove_duplicates_good(nums: &mut Vec<i32>) -> i32 {
    let mut head = 0;

    for i in 1..nums.len() {
        let num = nums[i];

        if nums[head] != num {
            head += 1;
            nums[head] = num;
        }
    }

    (head + 1) as i32
}

fn main() {
    let mut nums = vec![1, 1, 2];
    let len = remove_duplicates_good(&mut nums);
    println!("len: {}", len);
    println!("nums: {nums:?}");

    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let len = remove_duplicates_good(&mut nums);
    println!("len: {}", len);
    println!("nums: {nums:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut nums = vec![1, 1, 2];
        let len = remove_duplicates_bad(&mut nums);
        assert_eq!(len, 2);
        assert_eq!(nums, vec![1, 2, 0]);
    }

    #[test]
    fn test_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let len = remove_duplicates_bad(&mut nums);
        assert_eq!(len, 5);
        assert_eq!(nums, vec![0, 1, 2, 3, 4, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_3() {
        let mut nums = vec![1, 1, 1];
        let len = remove_duplicates_bad(&mut nums);
        assert_eq!(len, 1);
        assert_eq!(nums, vec![1, 0, 0]);
    }
}
