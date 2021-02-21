use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_match(string: String, pattern: String) -> bool {
        let mut dp = vec![vec![false; pattern.len() + 1]; string.len() + 1];
        let string = string.bytes().map(|ch| ch as char).collect::<Vec<_>>();
        let pattern = pattern.bytes().map(|ch| ch as char).collect::<Vec<_>>();

        dp[0][0] = true;
        for j in 1..pattern.len() + 1 {
            if pattern[j - 1] == '*' {
                dp[0][j] = dp[0][j - 2];
            }
        }
        for i in 1..string.len() + 1 {
            for j in 1..pattern.len() + 1 {
                dp[i][j] = match (string[i - 1], pattern[j - 1]) {
                    (ch, '*') if !vec![ch, '.'].contains(&pattern[j - 2]) => dp[i][j - 2],
                    (_, '*') => dp[i][j - 2] || dp[i - 1][j] || dp[i - 1][j - 2],
                    (_, '.') => dp[i - 1][j - 1],
                    (ch1, ch2) if ch1 == ch2 => dp[i - 1][j - 1],
                    _ => false,
                };
            }
        }

        return dp[string.len()][pattern.len()];
    }
}
