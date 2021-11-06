use crate::lc::Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sum = ((n+1)*n/2) as i32;
        for i in 0..n {
            sum -= nums[i];
        }
        sum
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::missing_number(vec![1]));
    assert_eq!(1, Solution::missing_number(vec![0]));
    assert_eq!(2, Solution::missing_number(vec![3,0,1]));
    assert_eq!(2, Solution::missing_number(vec![0,1]));
    assert_eq!(8, Solution::missing_number(vec![9,6,4,2,3,5,7,0,1]));
}