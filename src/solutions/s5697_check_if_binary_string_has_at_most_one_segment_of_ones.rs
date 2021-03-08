use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn check_ones_segment(string: String) -> bool {
        let mut state = 0;

        for byte in string.as_bytes() {
            state = match (state, byte) {
                (0, b'1') => 1,
                (1, b'0') => 2,
                (2, b'1') => {
                    return false;
                }
                _ => state,
            }
        }

        return true;
    }
}
