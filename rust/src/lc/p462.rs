use crate::lc::Solution;

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let v = nums[nums.len() / 2];
        nums.iter().map(|num|(num -  v).abs()).sum()
    }
}

#[test]
fn test() {
    assert_eq!(14, Solution::min_moves2(vec![1,0,0,8,6]));
    assert_eq!(2, Solution::min_moves2(vec![1,2,3]));
    assert_eq!(16, Solution::min_moves2(vec![1,10,2,9]));
    assert_eq!(15, Solution::min_moves2(vec![1,10,3,9]));
}