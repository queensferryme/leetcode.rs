use super::Solution;

use std::cmp::min;

impl Solution {
    #[allow(dead_code)]
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        let (mut left, mut right) = (1, max_sum + 1);
        let max_sum = max_sum as i64;

        while left < right {
            let mid = (left + right) / 2;
            if Solution::get_min_sum(n, index, mid) > max_sum {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return left - 1;
    }

    fn get_min_sum(n: i32, index: i32, value: i32) -> i64 {
        let (n, index, value) = (n as i64, index as i64, value as i64);
        let mut sum = n + value - 1;

        let len = min(index, value - 1);
        sum += (value * 2 - len - 1) * len / 2;
        sum -= len;

        let len = min(n - index - 1, value - 1);
        sum += (value * 2 - len - 1) * len / 2;
        sum -= len;

        return sum;
    }
}
