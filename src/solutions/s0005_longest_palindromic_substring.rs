use super::Solution;

use std::cmp::min;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_palindrome(string: String) -> String {
        let mut array = vec!['^', '#'];
        for char in string.chars() {
            array.push(char);
            array.push('#');
        }
        array.push('$');

        let (start, length) = Self::manachar(&array);

        return string.chars().skip(start).take(length).collect();
    }

    fn manachar(array: &Vec<char>) -> (usize, usize) {
        let mut lps = vec![0; array.len()];
        let (mut bound, mut center, mut index) = (0, 0, 0);
        for right in 1..array.len() - 1 {
            let left = 2 * center - right;
            if right < bound {
                lps[right] = min(bound - right, lps[left]);
            }
            if right > bound || lps[left] <= bound - right {
                while array[right + lps[right] + 1] == array[right - lps[right] - 1] {
                    lps[right] += 1;
                }
            }
            if right + lps[right] > bound {
                center = right;
                bound = right + lps[right];
            }
            if lps[right] > lps[index] {
                index = right;
            }
        }
        return ((index - lps[index] - 1) / 2, lps[index]);
    }
}
