use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn convert(string: String, n_rows: i32) -> String {
        if n_rows == 1 {
            return string;
        }

        let batch_size = n_rows as usize * 2 - 2;
        let bytes = string.as_bytes();
        let mut result = Vec::new();

        let mut i = 0;
        while i < string.len() {
            result.push(bytes[i]);
            i += batch_size;
        }

        for row in 1..n_rows - 1 {
            let mut i = row as usize;
            loop {
                if i < string.len() {
                    result.push(bytes[i])
                } else {
                    break;
                }
                if i + batch_size - 2 * (i % batch_size) < string.len() {
                    result.push(bytes[i + batch_size - 2 * (i % batch_size)])
                } else {
                    break;
                }
                i += batch_size;
            }
        }

        let mut i = n_rows as usize - 1;
        while i < string.len() {
            result.push(bytes[i]);
            i += batch_size;
        }

        return match String::from_utf8(result) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };
    }
}
