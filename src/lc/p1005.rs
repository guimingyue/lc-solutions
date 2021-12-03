use crate::lc::Solution;

/// #Vec, sort, binary_search, min
impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let search = nums.binary_search(&0).unwrap_or_else(|x| x);
        let mut lt_zero_count = 0;
        let len = search.min(nums.len()-1);
        for i in 0..=len {
            if nums[i] < 0 {
                lt_zero_count += 1;
                continue;
            }
            break;
        }

        let min = k.min(lt_zero_count);
        for i in 0..min {
            nums[i as usize] = -nums[i as usize];
        }

        nums.sort();
        if k > lt_zero_count && (k - lt_zero_count) % 2 == 1 {
            nums[0] = -nums[0]
        }

        nums.iter().sum()
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::largest_sum_after_k_negations(vec![-1], 1));
    assert_eq!(-1, Solution::largest_sum_after_k_negations(vec![1], 1));
    assert_eq!(1, Solution::largest_sum_after_k_negations(vec![1], 2));
    assert_eq!(0, Solution::largest_sum_after_k_negations(vec![1,-1], 2));
    assert_eq!(5, Solution::largest_sum_after_k_negations(vec![4,2,3], 1));
    assert_eq!(6, Solution::largest_sum_after_k_negations(vec![3,-1,0,2], 3));
    assert_eq!(13, Solution::largest_sum_after_k_negations(vec![2,-3,-1,5,-4], 2));
}