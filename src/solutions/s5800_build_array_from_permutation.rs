use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn build_array(numbers: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];

        for i in 0..numbers.len() {
            result.push(numbers[numbers[i] as usize]);
        }

        return result;
    }
}
