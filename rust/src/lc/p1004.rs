use crate::lc::Solution;

impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut sum = 0;
        let mut res = 0;
        while right < nums.len() {
            if nums[right] == 0 {
                sum += 1;
            }
            while sum > k {
                if nums[left] == 0 {
                    sum -= 1;
                }
                left += 1;
            }
            res = res.max(right - left + 1);
            right += 1;
        }
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(6, Solution::longest_ones(vec![1,1,1,0,0,0,1,1,1,1,0], 2));
    assert_eq!(10, Solution::longest_ones(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3));
    assert_eq!(1, Solution::longest_ones(vec![1], 1));
    assert_eq!(2, Solution::longest_ones(vec![0, 1], 1));
    assert_eq!(2, Solution::longest_ones(vec![0, 0, 1], 1));
}