#![allow(non_snake_case)]

use crate::solutions::Solution;

mod solutions;

fn main() {
    Solution::solve();
    Solution::length_of_longest_substring(String::from("abcabcbb"));
}
