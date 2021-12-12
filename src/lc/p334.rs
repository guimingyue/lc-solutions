use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut min, mut mid) = (i32::MAX, i32::MAX);
        for num in nums {
            if num > mid {
                return true;
            } else if num < min {
                min = num;
            } else if num > min && num < mid {
                mid = num;
            }
        }
        false
    }
}
#[test]
fn test() {
    assert!(!Solution::increasing_triplet(vec![1,1,-2,6]));
    assert!(Solution::increasing_triplet(vec![1,2,3,4,5]));
    assert!(Solution::increasing_triplet(vec![1,2,3]));
    assert!(Solution::increasing_triplet(vec![4,6,7]));
    assert!(!Solution::increasing_triplet(vec![5,4,3,2,1]));
    assert!(!Solution::increasing_triplet(vec![5,4,6,2,1]));
    assert!(Solution::increasing_triplet(vec![5,4,6,2,7]));
    assert!(Solution::increasing_triplet(vec![2,1,5,0,4,6]));
    assert!(Solution::increasing_triplet(vec![20,100,10,12,5,13]));
}