use crate::lc::Solution;

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let (mut min, mut max) = (i32::MAX, i32::MIN);
        for v in nums {
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
        }
        let v = (max - k) - (min + k);
        if v <= 0 {
            0
        } else {
            v
        }
    }
}
#[test]
fn test() {
    assert_eq!(0, Solution::smallest_range_i(vec![1], 0));
    assert_eq!(6, Solution::smallest_range_i(vec![0, 10], 2));
    assert_eq!(0, Solution::smallest_range_i(vec![1,3,6], 3));
    assert_eq!(0, Solution::smallest_range_i(vec![1,1,1], 3));
    assert_eq!(0, Solution::smallest_range_i(vec![1,2,3], 1));
    assert_eq!(0, Solution::smallest_range_i(vec![3,2,1], 1));
    assert_eq!(1, Solution::smallest_range_i(vec![3,2,1,0], 1));
    assert_eq!(0, Solution::smallest_range_i(vec![3,2,1,0], 3));
    assert_eq!(0, Solution::smallest_range_i(vec![3,2,1,0], 4));
}