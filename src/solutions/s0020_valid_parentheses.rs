use super::Solution;

use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn is_valid(string: String) -> bool {
        let map = [(')', '('), (']', '['), ('}', '{')]
            .iter()
            .cloned()
            .collect::<HashMap<_, _>>();
        let mut stack = Vec::new();

        for ch in string.chars() {
            match ch {
                '(' | '[' | '{' => stack.push(ch),
                _ => {
                    if map.get(&ch).map(|c| *c) != stack.pop() {
                        return false;
                    }
                }
            }
        }

        return stack.is_empty();
    }
}
