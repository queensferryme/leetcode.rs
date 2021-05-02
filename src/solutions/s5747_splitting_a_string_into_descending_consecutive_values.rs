use super::Solution;

use std::cmp::min;

impl Solution {
    #[allow(dead_code)]
    pub fn split_string(s: String) -> bool {
        let s = s.chars().map(|c| c as i64 - '0' as i64).collect::<Vec<_>>();

        return Solution::search(&s, 0, -1);
    }

    fn search(s: &Vec<i64>, start: usize, prev: i64) -> bool {
        if start == s.len() {
            return true;
        }

        let limit = min(s.len() as i64, s.len() as i64 + prev) as usize;
        let mut value = 0i64;
        for i in start..limit {
            value = value * 10 + s[i];
            if value > 10i64.pow(10) {
                break;
            } else if prev == -1 || prev == value + 1 {
                if Solution::search(s, i + 1, value) {
                    return true;
                }
            }
        }

        return false;
    }
}
