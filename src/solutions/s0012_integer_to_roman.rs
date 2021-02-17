use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn int_to_roman(mut number: i32) -> String {
        let combinations = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        let mut result = String::from("");

        for (integer, roman) in combinations.iter() {
            while number >= *integer {
                number -= *integer;
                result += *roman;
            }
        }

        return result;
    }
}
