use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_ice_cream(mut costs: Vec<i32>, mut coins: i32) -> i32 {
        let mut result = 0;
        costs.sort_unstable();

        for cost in costs {
            if coins >= cost {
                coins -= cost;
                result += 1;
            } else {
                break;
            }
        }

        return result;
    }
}
