use super::Solution;

use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut set = HashSet::new();

        for character in sentence.chars() {
            if !set.contains(&character) {
                set.insert(character);
            }
        }

        return set.len() == 26;
    }
}
