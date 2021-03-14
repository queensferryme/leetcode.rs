use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn are_almost_equal(string1: String, string2: String) -> bool {
        let (string1, string2) = (string1.as_bytes(), string2.as_bytes());
        let mut indices = vec![];

        for i in 0..string1.len() {
            if string1[i] == string2[i] {
                continue;
            } else if indices.len() == 2 {
                return false;
            } else {
                indices.push(i);
            }
        }

        return match indices.len() {
            0 => true,
            1 => false,
            2 => {
                string1[indices[0]] == string2[indices[1]]
                    && string1[indices[1]] == string2[indices[0]]
            }
            _ => panic!(),
        };
    }
}
