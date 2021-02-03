use std::cmp::max;
use std::collections::HashMap;

use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut map = HashMap::new();
        let mut result = 0;

        for right in 0..s.len() {
            let ch = s.as_bytes()[right] as char;
            if map.contains_key(&ch) {
                left = max(left, *map.get(&ch).unwrap() + 1);
            }
            result = max(result, right as i32 - left + 1);
            map.insert(ch, right as i32);
        }

        return result;
    }
}