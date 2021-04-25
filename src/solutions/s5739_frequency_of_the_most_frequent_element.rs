use super::Solution;

use std::cmp::{max, min};
use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn max_frequency(numbers: Vec<i32>, k: i32) -> i32 {
        let mut counter = HashMap::new();
        let mut result = 0;

        for number in numbers {
            *counter.entry(number).or_insert(0) += 1;
        }
        let mut array = counter.iter().map(|t| (*t.0, *t.1)).collect::<Vec<_>>();
        array.sort_by(|a, b| b.cmp(a));

        for i in 0..array.len() {
            let (mut k, number, mut sum) = (k, array[i].0, array[i].1);
            for j in i + 1..array.len() {
                let diff = number - array[j].0;
                let count = min(array[j].1, k / diff);
                if count == 0 {
                    break;
                } else {
                    k -= count * diff;
                    sum += count;
                }
            }
            result = max(result, sum);
        }

        return result;
    }
}
