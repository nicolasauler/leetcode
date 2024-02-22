#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

fn nested_sum(nested_list: Vec<NestedInteger>, layer: i32) -> i32 {
    let mut sum = 0;

    for nested_int in nested_list {
        match nested_int {
            NestedInteger::Int(value) => sum += value * layer,
            NestedInteger::List(list_val) => sum += nested_sum(list_val, layer + 1),
        }
    }

    return sum;
}

fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
    return nested_sum(nested_list, 1);
}

fn main() {
    let nested_list: Vec<NestedInteger> = vec![
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        NestedInteger::Int(2),
        NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
    ];
    let sum = depth_sum(nested_list);
    println!("sum: {sum}");
}
