use std::collections::VecDeque;

fn find_buildings(heights: Vec<i32>) -> Vec<i32> {
    let last = heights.len() - 1;
    let mut heights = heights;
    heights.reverse();
    let mut curr_max = 0;
    let mut buildings: VecDeque<i32> = VecDeque::with_capacity(heights.len());

    for i in 0..heights.len() {
        if heights[i] > curr_max {
            curr_max = heights[i];
            buildings.push_front((last - i) as i32);
        }
    }

    buildings.into()
}

fn main() {
    let heights = vec![4, 2, 3, 1];
    let result = find_buildings(heights);
    println!("{result:?}");
}
