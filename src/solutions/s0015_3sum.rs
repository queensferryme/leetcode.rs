use super::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn three_sum(mut numbers: Vec<i32>) -> Vec<Vec<i32>> {
        let mut left = 0;
        let mut result = vec![];

        numbers.sort();

        while left + 2 < numbers.len() && numbers[left] <= 0 {
            let (mut center, mut right) = (left + 1, numbers.len() - 1);
            while center < right {
                let sum = numbers[left] + numbers[center] + numbers[right];
                if sum == 0 {
                    result.push(vec![numbers[left], numbers[center], numbers[right]]);
                    center = Solution::next(&numbers, &center, 1);
                } else if sum < 0 {
                    center = Solution::next(&numbers, &center, 1);
                } else {
                    right = Solution::next(&numbers, &right, -1);
                }
            }
            left = Solution::next(&numbers, &left, 1);
        }

        return result;
    }

    fn next(numbers: &Vec<i32>, index: &usize, step: i32) -> usize {
        let mut index = *index;
        let value = numbers[index];
        while numbers.get(index) == Some(&value) {
            index = (index as i32 + step) as usize;
        }
        return if index == usize::MAX { 0 } else { index };
    }
}
