fn remove_duplicates(nums: &mut Vec<i32>, max: usize) -> i32 {
    let mut head = 0;
    let mut count = 1;

    for i in 1..nums.len() {
        let num = nums[i];

        if nums[head] != num {
            head += 1;
            nums[head] = num;
            count = 1;
        } else {
            count += 1;
            if count == max {
                head += 1;
                nums[head] = num;
            }
        }
    }

    (head + 1) as i32
}

fn main() {
    //let mut nums = vec![1, 1, 1, 2, 2, 3];
    //let count = remove_duplicates(&mut nums, 2);
    //println!("nums: {nums:?}");
    //println!("count: {count}");

    let mut nums = vec![0,0,1,1,1,1,2,3,3];
    let count = remove_duplicates(&mut nums, 2);
    println!("nums: {nums:?}");
    println!("count: {count}");
}
