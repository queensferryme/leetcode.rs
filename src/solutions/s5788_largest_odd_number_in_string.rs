use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn largest_odd_number(number: String) -> String {
        let mut last_index = -1;
        let chars = number.chars().collect::<Vec<_>>();

        for i in 0..number.len() {
            if chars[i].to_digit(10).unwrap() % 2 == 1 {
                last_index = i as i32;
            }
        }

        return number.chars().take((last_index + 1) as usize).collect();
    }
}
