use super::Solution;

use std::cmp::max;

impl Solution {
    #[allow(dead_code)]
    pub fn max_ascending_sum(numbers: Vec<i32>) -> i32 {
        let (mut last, mut result, mut sum) = (0, 0, 0);

        for number in numbers {
            if number > last {
                sum += number;
            } else {
                result = max(result, sum);
                sum = number;
            }
            last = number;
        }

        return max(result, sum);
    }
}
