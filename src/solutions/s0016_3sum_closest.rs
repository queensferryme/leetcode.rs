use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn three_sum_closest(mut numbers: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut result = 100000;

        numbers.sort();

        while left + 2 < numbers.len() {
            let (mut center, mut right) = (left + 1, numbers.len() - 1);
            while center < right {
                let sum = numbers[left] + numbers[center] + numbers[right];
                result = if (result - target).abs() > (sum - target).abs() {
                    sum
                } else {
                    result
                };
                if sum == target {
                    return target;
                } else if sum < target {
                    center += 1;
                } else {
                    right -= 1;
                }
            }
            left += 1;
        }

        return result;
    }
}
