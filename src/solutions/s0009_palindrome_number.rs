use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let (xo, mut xr) = (x, 0);
        while x != 0 {
            xr = xr * 10 + x % 10;
            x /= 10;
        }
        return xo == xr;
    }
}
