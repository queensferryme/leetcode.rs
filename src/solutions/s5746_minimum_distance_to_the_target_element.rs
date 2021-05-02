use super::Solution;

use std::cmp::min;

impl Solution {
    #[allow(dead_code)]
    pub fn get_min_distance(numbers: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut result = i32::MAX;

        for i in 0..numbers.len() {
            if numbers[i] == target {
                result = min(result, (i as i32 - start).abs());
            }
        }

        return result;
    }
}
