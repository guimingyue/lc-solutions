use crate::lc::Solution;

/// #Vec
impl Solution {

    pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
        const MOD:i32 = 1_000_000_007;
        let mut dp = vec![vec![0; k as usize + 1]; n as usize + 1];
        dp[0][0] = 1;
        let num = n as usize;
        let k_num = k as usize;
        for i in 1..=num {
            for j in 0..=k_num {
                if j < 1 {
                    dp[i][j] = dp[i-1][j];
                } else if j >= 1 && j < i {
                    dp[i][j] = dp[i][j-1] + dp[i-1][j];
                } else {
                    dp[i][j] = dp[i][j-1] - dp[i-1][j-i] + dp[i-1][j];
                }
                if dp[i][j] >= MOD {
                    dp[i][j] -= MOD;
                } else if dp[i][j] < 0 {
                    dp[i][j] += MOD;
                }
            }
        }
        dp[num][k_num]
    }

    pub fn k_inverse_pairs_tle(n: i32, k: i32) -> i32 {
        const MOD:i32 = 1_000_000_007;
        let mut dp = vec![vec![0; k as usize + 1]; n as usize + 1];
        dp[0][0] = 1;
        let num = n as usize;
        let k_num = k as usize;
        for i in 1..=num {
            for j in 0..=k_num {
                for l in 0..i {
                    if j < l  {
                        continue;
                    }
                    dp[i][j] += dp[i-1][j-l];
                    dp[i][j] %= MOD;
                }
            }
        }
        dp[num][k_num]
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::k_inverse_pairs(3, 0));
    assert_eq!(2, Solution::k_inverse_pairs(3, 1));
    assert_eq!(2, Solution::k_inverse_pairs(3, 2));
    assert_eq!(1, Solution::k_inverse_pairs(3, 3));
    assert_eq!(663677020, Solution::k_inverse_pairs(1000, 1000));
}