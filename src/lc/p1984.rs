use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let mut res = 100000;
        for i in (k-1) as usize..nums.len() {
            res = res.min(nums[i] - nums[i + 1 - k as usize]);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::minimum_difference(vec![90], 1));
    assert_eq!(2, Solution::minimum_difference(vec![9,4,1,7], 2));
    assert_eq!(5, Solution::minimum_difference(vec![9,4,1,7], 3));
}