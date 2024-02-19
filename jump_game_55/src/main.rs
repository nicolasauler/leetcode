fn can_jump(nums: Vec<i32>) -> bool {
    let mut jumps = 0;

    for i in 0..nums.len() {
        let jump = nums[i];

        if jump > jumps {
            jumps = jump;
        }
        if jumps == 0 && i != nums.len() - 1 {
            return false;
        }

        jumps -= 1;
    }

    return true;
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];
    let could = can_jump(nums);
    println!("could jump: {could}");

    let nums = vec![3, 2, 1, 0, 4];
    let could = can_jump(nums);
    println!("could jump: {could}");
}
