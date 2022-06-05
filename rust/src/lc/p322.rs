use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        const MAX:i32 = 10001;
        let mut dp = vec![MAX; amount as usize + 1];
        dp[0] = 0;
        for v in coins {
            for i in v..=amount {
                dp[i as usize] = dp[i as usize].min(dp[(i - v) as usize] + 1)
            }
        }
        if dp[amount as usize] == MAX {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::coin_change(vec![1, 2, 5], 11));
    assert_eq!(-1, Solution::coin_change(vec![2], 3));
    assert_eq!(0, Solution::coin_change(vec![1], 0));
    assert_eq!(1, Solution::coin_change(vec![1], 1));
    assert_eq!(2, Solution::coin_change(vec![1], 2));
    assert_eq!(4, Solution::coin_change(vec![2, 5, 10, 1], 27));
    assert_eq!(20, Solution::coin_change(vec![186,419,83,408], 6249));
}