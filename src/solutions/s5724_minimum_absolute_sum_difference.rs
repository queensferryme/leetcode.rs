use super::Solution;

use std::cmp::max;
use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn min_absolute_sum_diff(numbers1: Vec<i32>, numbers2: Vec<i32>) -> i32 {
        const MOD: i32 = 1000000007;
        let mut best_replacement = 0;
        let mut sum = 0;
        let mut set = numbers1
            .clone()
            .into_iter()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        set.sort();

        for i in 0..numbers1.len() {
            let difference = (numbers1[i] - numbers2[i]).abs();
            sum = (sum + difference) % MOD;
        }

        for i in 0..numbers2.len() {
            let current_difference = (numbers1[i] - numbers2[i]).abs();
            let index = match set.binary_search(&numbers2[i]) {
                Ok(i) => i,
                Err(i) => i,
            };

            if index != set.len() {
                let replaced_difference = (set[index] - numbers2[i]).abs();
                best_replacement = max(best_replacement, current_difference - replaced_difference);
            }

            if index != 0 {
                let replaced_difference = (set[index - 1] - numbers2[i]).abs();
                best_replacement = max(best_replacement, current_difference - replaced_difference);
            }
        }

        return (sum - best_replacement) % MOD;
    }
}
