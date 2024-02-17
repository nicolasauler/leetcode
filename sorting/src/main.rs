use std::fmt::Debug;

// Simple O(n^2) sorting algorithms

// In bubble sort, the ordered list is built bubbling the largest element to the end of the list
fn bubble_sort<T: Clone + Ord + Debug>(vec: &mut Vec<T>) {
    let len = vec.len();
    for i in 0..len {
        for j in 0..(len - i - 1) {
            if vec[j] > vec[j + 1] {
                let tmp = vec[j].clone();
                vec[j] = vec[j + 1].clone();
                vec[j + 1] = tmp;
            }
        }
    }
}

// Select sort divides the array into an ordered and an unordered subarray.
// It then takes the minimum value from the unordered and appends it to the ordered.
fn select_sort<T: Clone + Ord + Debug>(vec: &mut Vec<T>) {
    let len = vec.len();
    for i in 0..len {
        let mut min_idx = i;
        for j in i..(len - 1) {
            if vec[j + 1] < vec[min_idx] {
                min_idx = j + 1;
            }
        }

        let tmp = vec[i].clone();
        vec[i] = vec[min_idx].clone();
        vec[min_idx] = tmp;
    }
}

fn rusty_select_sort<T: Clone + Ord + Debug>(vec: &mut Vec<T>) {
    for i in 0..vec.len() {
        let min = vec[i..]
            .iter()
            .enumerate()
            .min_by_key(|t| t.1)
            .expect("there were no values")
            .0;

        vec.swap(i, min + i);
    }
}

// In insertion sort, the ordered list is built by constructing a sublist
// at the beginning, then adding elements to their respective position
fn insert_sort<T: Clone + Ord + Debug>(vec: &mut Vec<T>) {
    for i in 1..vec.len() {
        let value = vec[i].clone();
        for j in (0..i).rev() {
            if vec[j] > vec[i] {
                vec[j + 1] = vec[j].clone();
            } else {
                vec[j + 1] = value;
                break;
            }
        }
    }
}

// O(nlogn) sorting algorithms
fn rec_merge_sort<T: Clone + Ord + Debug>(vec: &Vec<T>) -> Vec<T> {
    let len = vec.len();
    if len < 2 {
        return vec.to_owned();
    } else {
        return merge(
            &rec_merge_sort(&vec[..(len / 2)].to_owned()),
            &rec_merge_sort(&vec[(len / 2)..].to_owned()),
        );
    }
}

fn ite_merge_sort() {
}

fn merge<T: Clone + Ord + Debug>(left: &Vec<T>, right: &Vec<T>) -> Vec<T> {
    let mut answer: Vec<T> = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            answer.push(left[i].clone());
            i += 1;
        } else {
            answer.push(right[j].clone());
            j += 1;
        }
    }

    while i < left.len() {
        answer.push(left[i].clone());
        i += 1;
    }
    while j < right.len() {
        answer.push(right[j].clone());
        j += 1;
    }

    return answer;
}

fn quick_sort() {
    todo!()
}

#[allow(dead_code)]
fn heap_sort() {
    todo!()
}

// O(n) sorting algorithms
#[allow(dead_code)]
fn counting_sort() {
    todo!()
}

#[allow(dead_code)]
fn radix_sort() {
    todo!()
}

fn main() {
    let mut vec = vec![1, 3, 2, 5, 4];
    println!("{:?}", vec);
    let new_vec = rec_merge_sort(&mut vec);
    println!("{:?}", new_vec);
}

#[allow(dead_code)]
fn test() {
    let mut vec = vec![1, 3, 2, 5, 4];
    println!("{:?}", vec);
    bubble_sort(&mut vec);
    println!("{:?}", vec);

    let mut vec = vec![1, 3, 2, 5, 4];
    println!("{:?}", vec);
    select_sort(&mut vec);
    println!("{:?}", vec);

    let mut vec = vec![1, 3, 2, 5, 4];
    println!("{:?}", vec);
    rusty_select_sort(&mut vec);
    println!("{:?}", vec);

    let mut vec = vec![1, 3, 2, 5, 4];
    println!("{:?}", vec);
    insert_sort(&mut vec);
    println!("{:?}", vec);
}
