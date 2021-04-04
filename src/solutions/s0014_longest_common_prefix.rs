use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_common_prefix(strings: Vec<String>) -> String {
        let mut length = 0;
        let strings = strings.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();

        if strings.len() == 0 {
            return String::from("");
        }

        loop {
            let ch = strings[0].get(length);
            for string in &strings {
                if ch != string.get(length) || ch.is_none() {
                    return String::from(match std::str::from_utf8(&strings[0][0..length]) {
                        Ok(v) => v,
                        Err(e) => panic!("{}", e),
                    });
                }
            }
            length += 1;
        }
    }
}
