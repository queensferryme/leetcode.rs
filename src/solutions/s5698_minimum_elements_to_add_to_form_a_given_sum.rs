use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_elements(numbers: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let (goal, limit) = (goal as i64, limit as i64);
        let sum = numbers.iter().map(|x| *x as i64).sum::<i64>();
        let diff = (sum - goal).abs();
        return (diff / limit) as i32 + (diff % limit != 0) as i32;
    }
}
