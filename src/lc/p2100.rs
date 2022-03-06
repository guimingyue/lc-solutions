use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        if security.len() <= time as usize {
            return vec![];
        }
        let mut res = vec![];
        let (mut left, mut right) = (vec![0; security.len()], vec![0; security.len()]);
        for i in 1..security.len() {
            left[i] = if security[i] <= security[i-1] {
                left[i-1] + 1
            } else {
                0
            };
            right[security.len() - 1 - i] = if security[security.len() -i] >= security[security.len() - i - 1] {
                right[security.len() - i] + 1
            } else {
                0
            };
        }
        for i in time as usize..security.len() - time as usize {
            if left[i] >= time && right[i] >= time {
                res.push(i as i32);
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Vec::<i32>::new(), Solution::good_days_to_rob_bank(vec![1], 5));
    assert_eq!(Vec::<i32>::new(), Solution::good_days_to_rob_bank(vec![1,2], 1));
    assert_eq!(vec![2,3], Solution::good_days_to_rob_bank(vec![5,3,3,3,5,6,2], 2));
    assert_eq!(vec![0, 1, 2, 3, 4], Solution::good_days_to_rob_bank(vec![1,1,1,1,1], 0));
    assert_eq!(Vec::<i32>::new(), Solution::good_days_to_rob_bank(vec![1,2,3,4,5,6], 2));
}