#![allow(non_snake_case)]

use crate::solutions::Solution;

mod solutions;

fn main() {
    Solution::solve();
    println!("{}", Solution::my_atoi(String::from("     -98")));
}
