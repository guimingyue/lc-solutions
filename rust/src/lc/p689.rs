use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let (mut sum1, mut max_sum1, mut max_sum1_idx) = (0, 0, 0);
        let (mut sum2, mut max_sum12, mut max_sum2_idx1, mut max_sum2_idx2) = (0, 0, 0, 0);
        let (mut sum3, mut max_sum, mut max_sum3_idx) = (0, 0, 0);
        let mut res = vec![0;3];
        let k = k as usize;
        for i in 2 * k..nums.len() {
            sum1 += nums[i - 2 * k];
            sum2 += nums[i-k];
            sum3 += nums[i];
            if i >= 3 * k - 1 {
                if sum1 > max_sum1 {
                    max_sum1 = sum1;
                    max_sum1_idx = ((i as i32 - 3 * k as i32) + 1) as usize;
                }

                if max_sum1 + sum2 > max_sum12 {
                    max_sum12 = max_sum1 + sum2;
                    max_sum2_idx1 = max_sum1_idx;
                    max_sum2_idx2 = i - 2 * k + 1;
                }

                if max_sum12 + sum3 > max_sum {
                    max_sum = max_sum12 + sum3;
                    res[0] = max_sum2_idx1 as i32;
                    res[1] = max_sum2_idx2 as i32;
                    res[2] = (i - k + 1) as i32;
                }

                sum1 -= nums[((i as i32 - 3 * k as i32) + 1) as usize];
                sum2 -= nums[i - 2 * k + 1];
                sum3 -= nums[i - k + 1];
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![0,3,5], Solution::max_sum_of_three_subarrays(vec![1,2,1,2,6,7,5,1], 2));
    assert_eq!(vec![0,2,4], Solution::max_sum_of_three_subarrays(vec![1,2,1,2,1,2,1,2,1], 2));
}