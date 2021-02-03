use std::collections::HashMap;

use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index = 0;
        let mut map = HashMap::new();
        for number in numbers {
            if let Some(i) = map.get(&(target - number)) {
                return vec![*i, index];
            }
            map.insert(number, index);
            index += 1;
        }
        return vec![-1, -1];
    }
}
