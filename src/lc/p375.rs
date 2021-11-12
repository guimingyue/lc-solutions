use crate::lc::Solution;

/// #Vec, min, max
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let nl = n as usize;
        let mut dp = vec![vec![0; nl + 1]; nl + 1];
        for i in (1..nl).rev() {
            for j in (i+1)..=nl {
                let mut min_cost = i32::MAX;
                for k in i..j {
                    min_cost = min_cost.min(k as i32 + std::cmp::max(dp[i][k-1], dp[k+1][j]))
                }
                dp[i][j] = min_cost;
            }
        }
        dp[1][nl]
    }
}

#[test]
fn test() {
    assert_eq!(16, Solution::get_money_amount(10));
    assert_eq!(0, Solution::get_money_amount(1));
    assert_eq!(1, Solution::get_money_amount(2));
}