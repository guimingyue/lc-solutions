use crate::lc::Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let (mut f0, mut f1) = (0, 0);
        for (i, v) in nums.iter().enumerate() {
            sum += v;
            f0 += i as i32 * *v;
        }
        let mut max = f0;
        let n = nums.len();
        for i in 1..n {
            f1 = f0 + (sum - nums[n - i]) - (n - 1) as i32 * nums[n-i];
            if f1 > max {
                max = f1;
            }
            f0 = f1;
        }

        max
    }
}

#[test]
fn test() {
    assert_eq!(26, Solution::max_rotate_function(vec![4,3,2,6]));
    assert_eq!(0, Solution::max_rotate_function(vec![100]));
}