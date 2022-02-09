use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {

        let mut nums = nums;
        nums.sort();
        let (mut i, mut j) = (0, 1);
        let mut res = 0;
        while j < nums.len() {
            let v = nums[j] - nums[i];
            if v > k {
                i += 1;
            } else if v < k {
                j += 1;
            } else {
                let mut i_num = 1;
                let mut j_num = 1;
                while i < j && nums[i+1] == nums[i] {
                    i_num += 1;
                    i += 1;
                }
                while j < nums.len() - 1 && nums[j+1] == nums[j] {
                    j_num += 1;
                    j += 1;
                }
                res += i_num * j_num;
                i += 1;
                j += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::count_k_difference(vec![1,2,2], 1));
    assert_eq!(4, Solution::count_k_difference(vec![1,2,2,1], 1));
    assert_eq!(6, Solution::count_k_difference(vec![1,2,2,2,1], 1));
    assert_eq!(0, Solution::count_k_difference(vec![1,3], 3));
    assert_eq!(3, Solution::count_k_difference(vec![3,2,1,5,4], 2));
}