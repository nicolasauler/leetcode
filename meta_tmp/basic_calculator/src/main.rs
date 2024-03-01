#![allow(dead_code)]
//use leetcode_227::calculate;

mod leetcode_227;
mod leetcode_editorial;
mod math_fun;

fn main() {
    //let result = calculate("14/3*2".to_owned());
    //println!("{}", result);
    println!("{}", leetcode_editorial::calculate("3+2*2".to_owned()));
}
