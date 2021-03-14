use super::Solution;

use std::collections::HashSet;

impl Solution {
    #[allow(dead_code)]
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut visited = HashSet::new();

        for node in edges.iter().flatten() {
            if visited.contains(node) {
                return *node;
            } else {
                visited.insert(node);
            }
        }

        return 0;
    }
}
