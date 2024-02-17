fn remove_element_bad(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&v| v != val);
    nums.len() as i32
}
fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let val = 3;
    let result = remove_element_bad(&mut nums, val);
    println!("result: {:?}", result);
    println!("nums: {:?}", nums);
}
