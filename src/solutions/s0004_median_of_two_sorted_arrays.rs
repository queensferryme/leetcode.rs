use super::Solution;

use std::cmp::min;

impl Solution {
    #[allow(dead_code)]
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (nums1, nums2) = (nums1.as_slice(), nums2.as_slice());
        let len = nums1.len() + nums2.len();

        return match len % 2 == 1 {
            true => Self::find_kth_sorted_arrays(nums1, nums2, len / 2 + 1),
            false => {
                (Self::find_kth_sorted_arrays(nums1, nums2, len / 2)
                    + Self::find_kth_sorted_arrays(nums1, nums2, len / 2 + 1))
                    / 2.0
            }
        };
    }

    fn find_kth_sorted_arrays(mut nums1: &[i32], mut nums2: &[i32], mut k: usize) -> f64 {
        loop {
            if nums1.len() == 0 || nums2.len() == 0 || k == 1 {
                return *min(
                    nums1.get(k - 1).unwrap_or(&i32::MAX),
                    nums2.get(k - 1).unwrap_or(&i32::MAX),
                ) as f64;
            }

            let idx = k / 2 - 1;
            let idx1 = min(idx, nums1.len() - 1);
            let idx2 = min(idx, nums2.len() - 1);
            if nums1[idx1] < nums2[idx2] {
                nums1 = &nums1[idx1 + 1..];
                k -= idx1 + 1;
            } else {
                nums2 = &nums2[idx2 + 1..];
                k -= idx2 + 1;
            }
        }
    }
}
