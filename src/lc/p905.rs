use crate::lc::Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            while l < r && nums[l] % 2 == 0 {
                l += 1;
            }
            while l < r && nums[r] % 2 != 0 {
                r -= 1;
            }
            nums.swap(l, r);
        }
        nums
    }
}

#[test]
fn test() {
    assert_eq!(vec![4, 2, 1,3], Solution::sort_array_by_parity(vec![3,1,2,4]));
    assert_eq!(vec![0], Solution::sort_array_by_parity(vec![0]));
    assert_eq!(vec![0, 1], Solution::sort_array_by_parity(vec![0, 1]));
    assert_eq!(vec![0, 1, 3], Solution::sort_array_by_parity(vec![0, 1, 3]));
    assert_eq!(vec![0, 2, 1], Solution::sort_array_by_parity(vec![0, 1, 2]));
}