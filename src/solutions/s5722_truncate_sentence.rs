use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn truncate_sentence(string: String, mut k: i32) -> String {
        let mut n = 0;

        for char in string.chars() {
            k -= (char == ' ') as i32;
            if k == 0 {
                break;
            } else {
                n += 1;
            }
        }

        return string.chars().take(n).collect();
    }
}
