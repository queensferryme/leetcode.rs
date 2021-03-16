use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut results = vec![String::from("")];

        for digit in digits.chars() {
            let mut new_results = vec![];
            for letter in Solution::letters(digit).chars() {
                for result in &results {
                    new_results.push(format!("{}{}", result, letter));
                }
            }
            results = new_results;
        }

        return if digits.is_empty() { vec![] } else { results };
    }

    fn letters(digit: char) -> &'static str {
        return match digit {
            '2' => "abc",
            '3' => "def",
            '4' => "ghi",
            '5' => "jkl",
            '6' => "mno",
            '7' => "pqrs",
            '8' => "tuv",
            '9' => "wxyz",
            _ => panic!(),
        };
    }
}
