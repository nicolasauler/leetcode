#![allow(dead_code)]

fn rotate_left(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let mut tmp = vec![0; k as usize];
    for i in 0..k as usize {
        tmp[i] = nums[i];
    }
    for i in 0..(len - k as usize) {
        nums[i] = nums[i + k as usize];
    }
    for i in 0..k as usize {
        nums[len - k as usize + i] = tmp[i];
    }
}

fn rotate_right(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let mut tmp = vec![0; k as usize];
    for i in 0..k as usize {
        tmp[i] = nums[len - k as usize + i];
    }
    for i in (k as usize..len).rev() {
        nums[i] = nums[i - k as usize];
    }
    println!("{:?}", tmp);
    for i in 0..k as usize {
        nums[i] = tmp[i];
    }
}

fn rotate_left_without_extra_space(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let mut start = 0;

    while start < len - k as usize {
        for mut i in start..(start + k as usize) {
            if i + k as usize >= len {
                let num = k as usize - (i % k as usize);
                for _ in 0..num {
                    while i < len - 1 {
                        nums.swap(i, i + 1);
                        i += 1;
                    }
                }
                break;
            }
            nums.swap(i, i + k as usize);
        }
        start += k as usize;
    }
}

fn rotate_right_without_extra_space(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    let mut start = len - 1;

    while start >= k as usize {
        for mut i in ((start - k as usize + 1)..=start).rev() {
            if i as i32 - k < 0 {
                let num = k as usize - (start - i);
                let old_i = i;
                for _ in 0..num {
                    while i > 0 {
                        nums.swap(i, i - 1);
                        i -= 1;
                    }
                    i = old_i;
                }
                break;
            }
            nums.swap(i, i - k as usize);
        }
        start -= k as usize;
    }
}

fn main() {
    //    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    //    let k = 3;
    //    rotate_left(&mut nums, k);
    //    println!("{:?}", nums);
    //
    //    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    //    let k = 3;
    //    rotate_right(&mut nums, k);
    //    println!("{:?}", nums);
    //
    //    let mut nums = vec![-1];
    //    let k = 2;
    //    rotate_right(&mut nums, k);
    //    println!("{:?}", nums);

    //    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    //    let k = 3;
    //    rotate_left_without_extra_space(&mut nums, k);
    //    println!("{:?}", nums);

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    rotate_right_without_extra_space(&mut nums, k);
    println!("{:?}", nums);
}
