//fn sort_arrow_shots(points: &mut [Vec<i32>]) {}

// if I sort the input, I then have
// [1,6], [2,8], [7,12], [10,16]
// [1,8], [2,6], [9,12], [10,16]
// so I can check: if the first element from next set is larger than
// the last element of curr set, then I can shoot
// at the last element of curr set
fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
    let mut points = points;
    points.sort_unstable();
    let mut i: usize = 0;
    let mut shots: Vec<usize> = Vec::new();

    while i < points.len() - 1 {
        let curr = &points[i];
        let mut shoot = curr[1];
        let mut next = &points[i + 1];

        while next[0] < shoot {
            if next[1] < shoot {
                shoot = next[1];
            }

            i += 1;
            if i == points.len() - 1 {
                i -= 1;
                break;
            }
            next = &points[i + 1];
        }
        shots.push(i);
        i += 1;
    }

    return shots.len() as i32;
}

fn main() {
    let points: Vec<Vec<i32>> = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
    let n_shots = find_min_arrow_shots(points);
    println!("{n_shots}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0() {
        let points: Vec<Vec<i32>> = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        let n_shots = find_min_arrow_shots(points);
        assert_eq!(2, n_shots);
    }

    #[test]
    fn test_1() {
        let points: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        let n_shots = find_min_arrow_shots(points);
        assert_eq!(4, n_shots);
    }

    #[test]
    fn test_2() {
        let points: Vec<Vec<i32>> = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        let n_shots = find_min_arrow_shots(points);
        assert_eq!(2, n_shots);
    }
}
