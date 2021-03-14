use super::Solution;

use std::cmp::{max, min};

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_score(numbers: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (k as usize, k as usize);
        let (mut minimum, mut result) = (numbers[left], numbers[left]);

        loop {
            match (left, right) {
                (0, r) if r + 1 == numbers.len() => {
                    break;
                }
                (0, _) => {
                    right += 1;
                    minimum = min(minimum, numbers[right]);
                }
                (_, r) if r + 1 == numbers.len() => {
                    left -= 1;
                    minimum = min(minimum, numbers[left]);
                }
                (l, r) if numbers[l - 1] < numbers[r + 1] => {
                    right += 1;
                    minimum = min(minimum, numbers[right]);
                }
                _ => {
                    left -= 1;
                    minimum = min(minimum, numbers[left]);
                }
            }
            result = max(result, minimum * (right - left + 1) as i32);
        }

        return result;
    }
}
