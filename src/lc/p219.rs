use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        for i in 0..nums.len() - 1 as usize {
            let max_j = std::cmp::min(i + k as usize + 1, nums.len());
            for j in i + 1..max_j as usize {
                if nums[i] == nums[j] {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::contains_nearby_duplicate(vec![99,99], 2));
    assert!(Solution::contains_nearby_duplicate(vec![1,2,3,1], 3));
    assert!(Solution::contains_nearby_duplicate(vec![1,0,1,1], 1));
    assert!(!Solution::contains_nearby_duplicate(vec![1,2,3,1,2,3], 2));
}