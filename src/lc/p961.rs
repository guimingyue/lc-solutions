use crate::lc::Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let (mut v1, v2) = (nums[0], nums[1]);
        for i in 2..nums.len() {
            if v1 == nums[i] {
                return v1;
            } else if v2 == nums[i] {
                return v2;
            } else {
                v1 = nums[i];
            }
        }
        v1
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::repeated_n_times(vec![1,2,5,2,3,2]));
    assert_eq!(3, Solution::repeated_n_times(vec![1,2,3,3]));
    assert_eq!(2, Solution::repeated_n_times(vec![2,1,2,5,3,2]));
    assert_eq!(5, Solution::repeated_n_times(vec![5,1,5,2,5,3,5,4]));
}