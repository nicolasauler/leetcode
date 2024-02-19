fn jump(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    if len == 1 {
        return 0;
    }

    let mut max_reachable: Vec<usize> = vec![0; len];
    max_reachable[0] = nums[0] as usize;

    for i in 1..len {
        max_reachable[i] = max_reachable[i - 1].max(i + nums[i] as usize);
    }

    let mut path = Vec::new();
    let mut i: i32 = len as i32 - 1;
    let mut end = len - 1;
    while i >= 0 {
        while i >= 0 && max_reachable[i as usize] >= end {
            i -= 1;
        }
        path.push(i + 1);
        end = (i + 1) as usize;
    }

    return path.len() as i32;
}

fn main() {
    let nums = vec![2, 3, 3, 1, 4];
    let jumps = jump(nums);
    println!("jumps: {jumps}");
}
