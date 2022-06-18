use std::collections::BTreeMap;
use crate::lc::Solution;

/// #BTreeMap
impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        fn binary_search(nums: &Vec<i32>, end: usize, target: i32) -> usize {
            let (mut left, mut right) = (0, end);
            while left < right {
                let mid = (left + right) / 2;
                if nums[mid] >= target {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left
        }

        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let (mut left, mut right) = (0, nums[n-1] - nums[0]);
        while left <= right {
            let mid = (left + right) / 2;
            let mut cnt = 0;
            for i in 0..n {
                let j = binary_search(&nums, i, nums[i] - mid);
                cnt += i - j;
            }
            if cnt as i32 >= k {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::smallest_distance_pair(vec![1, 6, 2, 2], 1));
    assert_eq!(1, Solution::smallest_distance_pair(vec![1, 6, 2], 1));
    assert_eq!(4, Solution::smallest_distance_pair(vec![1, 6, 2], 2));
    assert_eq!(5, Solution::smallest_distance_pair(vec![1, 6, 2], 3));
    assert_eq!(0, Solution::smallest_distance_pair(vec![1, 3, 1], 1));
    assert_eq!(0, Solution::smallest_distance_pair(vec![1, 1, 1], 2));
    assert_eq!(5, Solution::smallest_distance_pair(vec![1, 6, 1], 3));
}