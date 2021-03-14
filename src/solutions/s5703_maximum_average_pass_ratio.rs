use super::Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq)]
struct Float64(f64);

impl Eq for Float64 {}

impl Ord for Float64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Float64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn max_average_ratio(mut classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut heap = BinaryHeap::new();

        for i in 0..classes.len() {
            let (pass, total) = (classes[i][0], classes[i][1]);
            heap.push((Solution::delta(pass, total), i));
        }

        for _ in 0..extra_students {
            let (_, index) = heap.pop().unwrap();
            classes[index][0] += 1;
            classes[index][1] += 1;
            heap.push((Solution::delta(classes[index][0], classes[index][1]), index));
        }

        let mut result = 0.0;
        for class in &classes {
            result += class[0] as f64 / class[1] as f64;
        }
        return result / classes.len() as f64;
    }

    fn delta(pass: i32, total: i32) -> Float64 {
        let (pass, total) = (pass as f64, total as f64);
        let ratio_now = pass / total;
        let ratio_then = (pass + 1.0) / (total + 1.0);
        return Float64(ratio_then - ratio_now);
    }
}
