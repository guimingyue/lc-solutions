use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut min_nums = vec![];
        let mut min = i32::MAX;
        for v in &nums {
            if *v < min {
                min = *v;
            }
            min_nums.push(min);
        }
        let mut res = 0;
        for i in (0..nums.len()).rev() {
            let diff = nums[i] - min_nums[i];
            if diff > res {
                res = diff;
            }
        }
        if res == 0 {
            -1
        } else {
            res
        }
    }
}

#[test]
fn test() {
    assert_eq!(4, Solution::maximum_difference(vec![7,1,5,4]));
    assert_eq!(-1, Solution::maximum_difference(vec![9,4,3,2]));
    assert_eq!(9, Solution::maximum_difference(vec![1,5,2,10]));
}