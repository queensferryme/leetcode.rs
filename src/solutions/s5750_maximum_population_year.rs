use super::Solution;

use std::cmp::min;
use std::collections::HashMap;

impl Solution {
    #[allow(dead_code)]
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        let mut maximum = 0;
        let mut result = i32::MAX;

        for log in logs {
            for year in log[0]..log[1] {
                *map.entry(year).or_insert(0) += 1;
            }
        }

        for year in map.keys() {
            let population = *map.get(year).unwrap();
            if population > maximum {
                maximum = population;
                result = *year;
            } else if population == maximum {
                result = min(result, *year);
            }
        }

        return result;
    }
}
