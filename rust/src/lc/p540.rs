use crate::lc::Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] == nums[mid - 1] {
                if (r - (mid - 1) + 1) % 2 == 0 {
                    r = mid - 2;
                } else {
                    l = mid + 1;
                }
            } else if nums[mid] == nums[mid + 1] {
                if (r - mid + 1) % 2 == 0 {
                    r = mid - 1;
                } else {
                    l = mid + 2;
                }
            } else {
                return nums[mid]
            }
        }
        nums[l]
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::single_non_duplicate(vec![1,1,2,3,3,4,4,8,8]));
    assert_eq!(10, Solution::single_non_duplicate(vec![3,3,7,7,10,11,11]));
    assert_eq!(1, Solution::single_non_duplicate(vec![1]));
    assert_eq!(1, Solution::single_non_duplicate(vec![1,2,2]));
    assert_eq!(2, Solution::single_non_duplicate(vec![1,1,2]));
    assert_eq!(2, Solution::single_non_duplicate(vec![1,1,2,3,3]));
}