use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(string: String) -> i32 {
        let mut result = 0;
        let integers: Vec<i32> = string.chars().map(Solution::char_to_int).collect();

        for (first, second) in integers.iter().zip(integers.iter().skip(1)) {
            result += if first < second { -first } else { *first };
        }

        return result + integers.last().unwrap();
    }

    fn char_to_int(ch: char) -> i32 {
        return match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!(),
        };
    }
}
