use crate::lc::Solution;

/// #Vec, max
impl Solution {

    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut dp = vec![1;nums.len()];
        let mut res = 0;
        for i in 0..nums.len() {
            for j in 0..=i {
                if nums[i] > nums[j] {
                    dp[i] = std::cmp::max(dp[i], dp[j]+1);
                }
            }
            if res < dp[i] {
                res = dp[i];
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(4, Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]));
    assert_eq!(4, Solution::length_of_lis(vec![0,1,0,3,2,3]));
    assert_eq!(1, Solution::length_of_lis(vec![7,7,7,7,7,7,7]));
}