use super::Solution;

use std::cmp::max;

impl Solution {
    #[allow(dead_code)]
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut result = 0;
        let (mut left, mut right) = (0usize, heights.len() - 1);

        while left < right {
            if heights[left] < heights[right] {
                result = max(result, heights[left] * (right - left) as i32);
                left += 1;
            } else {
                result = max(result, heights[right] * (right - left) as i32);
                right -= 1;
            }
        }

        return result;
    }
}
