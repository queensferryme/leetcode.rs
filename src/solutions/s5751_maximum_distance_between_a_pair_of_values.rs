use super::Solution;

use std::cmp::max;

impl Solution {
    #[allow(dead_code)]
    pub fn max_distance(numbers1: Vec<i32>, numbers2: Vec<i32>) -> i32 {
        let mut result = 0;

        let mut j = 0;
        for i in 0..numbers1.len() {
            j = max(j, i + result + 1);
            while j < numbers2.len() {
                if numbers1[i] > numbers2[j] {
                    break;
                }
                result = max(result, j - i);
                j += 1;
            }
        }

        return result as i32;
    }
}
