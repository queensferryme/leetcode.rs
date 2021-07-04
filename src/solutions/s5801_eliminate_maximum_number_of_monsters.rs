use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn eliminate_maximum(distance: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut time = distance
            .iter()
            .zip(speed.iter())
            .map(|(d, s)| *d as f64 / *s as f64)
            .collect::<Vec<_>>();

        time.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        for i in 0..time.len() {
            if time[i] > i as f64 {
                result += 1
            } else {
                break;
            }
        }

        return result as i32;
    }
}
