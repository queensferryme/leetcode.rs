use super::Solution;

use std::cmp::max;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let mut result = 0;
        let mut state = ('#', 0);

        for character in word.chars() {
            state = match (state.0, character) {
                (c1, c2) if c1 == c2 => (c1, state.1 + 1),
                (_, 'a') => ('a', 1),
                (c1, c2) if (c2 as u32) - (c1 as u32) < 7 => (c2, state.1 + 1),
                _ => ('#', 0),
            };
            if state.0 == 'u' {
                result = max(result, state.1);
            }
        }

        return result;
    }
}
