use crate::lc::Solution;

/// #Vec, max, HashMap
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut res = 0;
        let mut dp = std::collections::HashMap::new();
        arr.iter().for_each(|v| {
            let pre = *v - difference;
            if let Some(cnt) = dp.get(&pre) {
                dp.insert(v, *cnt + 1);
            } else {
                dp.insert(v, 1);
            }
            let cur = dp.get(&v).unwrap();
            if *cur > res {
                res = *cur;
            }
        });

        res
    }
    pub fn longest_subsequence_dp(arr: Vec<i32>, difference: i32) -> i32 {
        let mut dp = vec![1; arr.len()];
        let mut res = 1;
        for i in 1..arr.len() {
            if i == 10 {
                println!("***");
            }
            for j in 0..i {
                if arr[j] + difference == arr[i] {
                    dp[i] = std::cmp::max(dp[i], dp[j] + 1);
                }
            }
            if res < dp[i] {
                res = dp[i]
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::longest_subsequence(vec![4,12,10,0,-2,7,-8,9,-9,-12,-12,8,8], 0));
    assert_eq!(4, Solution::longest_subsequence(vec![1,2,3,4], 1));
    assert_eq!(1, Solution::longest_subsequence(vec![1,3,5,7], 1));
    assert_eq!(4, Solution::longest_subsequence(vec![1,5,7,8,5,3,4,2,1], -2));
    assert_eq!(1, Solution::longest_subsequence(vec![1], 1));
    assert_eq!(2, Solution::longest_subsequence(vec![1,2], 1));
    assert_eq!(1, Solution::longest_subsequence(vec![1,3], 1));

}