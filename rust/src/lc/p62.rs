use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let (m_len, n_len) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n_len + 1]; m_len + 1];
        dp[1].fill(1);
        for i in 1..=m_len {
            dp[i][1] = 1;
        }
        for i in 2..=m_len {
            for j in 2..=n_len {
                dp[i][j] = dp[i-1][j] + dp[i][j-1];
            }
        }
        dp[m_len][n_len]
    }
    pub fn unique_paths_(m: i32, n: i32) -> i32 {
        let (m_len, n_len) = (m as usize, n as usize);
        let mut dp = vec![vec![0; n_len + 1]; m_len + 1];
        dp[1].fill(1);
        for i in 1..=m_len {
            dp[i][1] = 1;
        }
        for i in 2..=m_len {
            for j in 2..=n_len {
                for k in 1..=i {
                    dp[i][j] += dp[k][j-1];
                }
            }
        }
        dp[m_len][n_len]
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::unique_paths(3, 2));
    assert_eq!(28, Solution::unique_paths(3, 7));
    assert_eq!(28, Solution::unique_paths(7, 3));
    assert_eq!(6, Solution::unique_paths(3, 3));
    assert_eq!(4, Solution::unique_paths(4, 2));
}