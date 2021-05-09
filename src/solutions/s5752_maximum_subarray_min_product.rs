use super::Solution;

use std::cmp::max;

impl Solution {
    #[allow(dead_code)]
    pub fn max_sum_min_product(numbers: Vec<i32>) -> i32 {
        let mut result = 0i64;

        let mut left = vec![0usize; numbers.len()];
        let mut right = vec![numbers.len() - 1; numbers.len()];
        let mut stack: Vec<usize> = vec![];
        for i in 0..numbers.len() {
            while !stack.is_empty() && numbers[*stack.last().unwrap()] >= numbers[i] {
                right[*stack.last().unwrap()] = i - 1;
                stack.pop();
            }
            if !stack.is_empty() {
                left[i] = *stack.last().unwrap() + 1;
            }
            stack.push(i);
        }

        let mut prefix = vec![0i64; numbers.len() + 1];
        for i in 1..=numbers.len() {
            prefix[i] = prefix[i - 1] + numbers[i - 1] as i64;
        }

        for i in 0..numbers.len() {
            result = max(
                result,
                numbers[i] as i64 * (prefix[right[i] + 1] - prefix[left[i]]),
            );
        }

        return (result % 1000000007) as i32;
    }
}
