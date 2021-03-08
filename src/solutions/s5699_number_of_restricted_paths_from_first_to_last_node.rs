use super::Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    #[allow(dead_code)]
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; (n + 1) as usize];
        for edge in edges {
            graph[edge[0] as usize].push((edge[2], edge[1] as usize));
            graph[edge[1] as usize].push((edge[2], edge[0] as usize));
        }

        // dijkstra shortest path
        let mut distances = vec![i32::MAX; (n + 1) as usize];
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, n as usize)));
        distances[n as usize] = 0;
        while !heap.is_empty() {
            let Reverse((distance, node)) = heap.pop().unwrap();
            for edge in &graph[node] {
                if distance + edge.0 < distances[edge.1] {
                    distances[edge.1] = distance + edge.0;
                    heap.push(Reverse((distances[edge.1], edge.1)));
                }
            }
        }

        let mut counter = vec![0; (n + 1) as usize];
        let mut heap = BinaryHeap::new();
        let mut visited = vec![false; (n + 1) as usize];
        counter[1] = 1;
        heap.push((distances[1], 1));
        while !heap.is_empty() {
            let (_, node) = heap.pop().unwrap();
            if !visited[node] {
                visited[node] = true;
                for edge in &graph[node] {
                    if distances[edge.1] < distances[node] {
                        counter[edge.1] = (counter[edge.1] + counter[node]) % 1000000007;
                        heap.push((distances[edge.1], edge.1));
                    }
                }
            }
        }

        return counter[n as usize];
    }
}
