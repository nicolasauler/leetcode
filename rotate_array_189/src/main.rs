fn rotate(nums: &mut Vec<i32>, k: i32) {
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

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let k = 3;
    rotate(&mut nums, k);
    println!("{:?}", nums);
}
