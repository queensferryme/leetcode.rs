use super::Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    #[allow(dead_code)]
    pub fn count_different_subsequence_gc_ds(numbers: Vec<i32>) -> i32 {
        let mut result = 0;
        let set: HashSet<&i32> = HashSet::from_iter(&numbers);
        let maximum = *set.iter().max().unwrap_or(&&0);

        for divisor in 1..maximum + 1 {
            let mut greatest_common_divisor = 0;
            for multiple in (divisor..maximum + 1).step_by(divisor as usize) {
                if set.contains(&multiple) {
                    greatest_common_divisor = if greatest_common_divisor == 0 {
                        multiple
                    } else {
                        Solution::gcd(greatest_common_divisor, multiple)
                    };
                    if greatest_common_divisor == divisor {
                        result += 1;
                        break;
                    }
                }
            }
        }

        return result;
    }

    fn gcd(a: i32, b: i32) -> i32 {
        return if b == 0 { a } else { Solution::gcd(b, a % b) };
    }
}
