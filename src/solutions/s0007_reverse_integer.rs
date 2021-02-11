use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse(x: i32) -> i32 {
        let (mut result, mut x) = (0i64, x as i64);
        while x != 0 {
            result = result * 10 + x % 10;
            x /= 10;
            if result > i32::MAX as i64 || result < i32::MIN as i64 {
                return 0;
            }
        }
        return result as i32;
    }
}
