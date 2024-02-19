fn is_power_of_two_lazy(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    n.count_ones() == 1
}

fn is_power_of_two_less_lazy(n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    (n & (n - 1)) == 0
    // (8) 1000 &
    // (7) 0111 =
    // (0) 0000
}

fn main() {
    let n: i32 = -2147483648;
    // show n bits
    println!("{:b}", n);
}
