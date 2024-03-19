struct SparseElement {
    index: usize,
    value: i32,
}

struct SparseVector {
    list: Vec<SparseElement>,
}

impl SparseVector {
    fn new(nums: Vec<i32>) -> Self {
        let mut sparse_vector = Vec::new();
        for (i, num) in nums.iter().enumerate() {
            if *num != 0 {
                sparse_vector.push(SparseElement {
                    index: i,
                    value: *num,
                });
            }
        }
        Self {
            list: sparse_vector,
        }
    }

    fn dot_product(&self, vec: SparseVector) -> i32 {
    }
}

fn main() {
    let s_vec1 = SparseVector::new(vec![1, 0, 0, 2, 3]);
    let s_vec2 = SparseVector::new(vec![0, 3, 0, 4, 0]);
    let product = s_vec1.dot_product(s_vec2);
    println!("{product}");
}
