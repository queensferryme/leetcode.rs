use super::Solution;

use std::cmp::min;

fn find_kth_sorted_arrays(mut nums1: &[i32], mut nums2: &[i32], mut k: usize) -> f64 {
    loop {
        if nums1.len() == 0 {
            return nums2[k - 1] as f64;
        } else if nums2.len() == 0 {
            return nums1[k - 1] as f64;
        } else if k == 1 {
            return min(nums1[0], nums2[0]) as f64;
        }

        let idx = k / 2 - 1;
        let idx1 = if idx < nums1.len() {
            idx
        } else {
            nums1.len() - 1
        };
        let idx2 = if idx < nums2.len() {
            idx
        } else {
            nums2.len() - 1
        };
        if nums1[idx1] < nums2[idx2] {
            nums1 = &nums1[idx1 + 1..];
            k -= idx1 + 1;
        } else {
            nums2 = &nums2[idx2 + 1..];
            k -= idx2 + 1;
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = (nums1.as_slice(), nums2.as_slice());
        let len = nums1.len() + nums2.len();

        return match len % 2 == 1 {
            true => find_kth_sorted_arrays(nums1, nums2, len / 2 + 1),
            false => {
                (find_kth_sorted_arrays(nums1, nums2, len / 2)
                    + find_kth_sorted_arrays(nums1, nums2, len / 2 + 1))
                    / 2.0
            }
        };
    }
}
