use super::Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    #[allow(dead_code)]
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut user: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut result = vec![0; k as usize];

        for log in &logs {
            user.entry(log[0]).or_default().insert(log[1]);
        }

        for (_, set) in user.into_iter() {
            result[set.len() - 1] += 1;
        }

        return result;
    }
}
