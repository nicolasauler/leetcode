fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i: usize = 0;
    let mut j: usize = 0;

    while i < m as usize && j < n as usize {
        if nums1[i + j] <= nums2[j] {
            i += 1;
        } else {
            nums1.insert(i + j, nums2[j]);
            nums1.pop();
            j += 1;
        }
    }

    while i + j < (m + n) as usize && j < n as usize {
        nums1[i + j] = nums2[j];
        j += 1;
    }
}

fn main() {
    let mut vec = vec![4, 0, 0, 0, 0, 0];
    let mut vec2 = vec![1, 2, 3, 5, 6];
    //let mut vec = vec![1, 2, 3, 0, 0, 0];
    //let mut vec2 = vec![2, 5, 6];
    merge(&mut vec, 1, &mut vec2, 5);
    println!("{:?}", vec);
}
