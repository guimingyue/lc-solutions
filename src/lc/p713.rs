use crate::lc::Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let  mut i = 0;
        let mut res = 0;
        let mut prod = 1;
        for j in 0..nums.len() {
            prod *= nums[j];
            while i <= j && prod >= k {
                prod /= nums[i];
                i += 1;
            }
            res += j as i32 - i as i32 + 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(8, Solution::num_subarray_product_less_than_k(vec![10,5,2,6], 100));
    assert_eq!(0, Solution::num_subarray_product_less_than_k(vec![1,2,3], 0));
}