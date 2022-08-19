use std::collections::HashMap;
use crate::lc::Solution;

impl Solution {
    pub fn max_equal_freq(nums: Vec<i32>) -> i32 {
        let mut count:HashMap<i32, i32> = HashMap::new();
        let mut freq:HashMap<i32, i32> = HashMap::new();
        let (mut res, mut maxFreq) = (0, 0);
        for i in 0..nums.len() {
            let v = nums[i];
            let c = count.entry(v).or_default();
            if *c > 0 {
                let freq_c = freq.entry(*c).or_default();
                *freq_c -= 1;
            }
            *c += 1;
            maxFreq = maxFreq.max(*c);
            let freq_c = freq.entry(*c).or_default();
            *freq_c += 1;

            let maxFreq_mimus_1 = if let Some(c) = freq.get(&(maxFreq - 1)) {
                *c
            } else {
                0
            };

            if maxFreq == 1 ||
                maxFreq + maxFreq_mimus_1 * (maxFreq - 1) == i as i32 + 1 && freq[&maxFreq] == 1 ||
                freq[&maxFreq] * maxFreq == i as i32 && freq[&1] == 1 {
                res = i + 1
            }
        }

        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::max_equal_freq(vec![1, 1]));
    assert_eq!(3, Solution::max_equal_freq(vec![1, 1, 1]));
    assert_eq!(3, Solution::max_equal_freq(vec![1, 1, 2]));
    assert_eq!(7, Solution::max_equal_freq(vec![2,2,1,1,5,3,3,5]));
    assert_eq!(13, Solution::max_equal_freq(vec![1,1,1,2,2,2,3,3,3,4,4,4,5]));
}