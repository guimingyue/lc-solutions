use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; n+1]; m+1];
        for j in 1..=n {
            dp[1][j] = dp[1][j-1] + grid[0][j-1];
        }
        for i in 1..=m {
            dp[i][1] = dp[i-1][1] + grid[i-1][0];
        }
        for i in 2..=m {
            for j in 2..=n {
                dp[i][j] = std::cmp::min(dp[i-1][j], dp[i][j-1]) + grid[i-1][j-1];
            }
        }
        dp[m][n]
    }
}

#[test]
fn test() {
    assert_eq!(7, Solution::min_path_sum(vec![vec![1,3,1], vec![1,5,1], vec![4,2,1]]));
    assert_eq!(12, Solution::min_path_sum(vec![vec![1,2,3], vec![4,5,6]]));
    assert_eq!(21, Solution::min_path_sum(vec![vec![1,2,3,4,5,6]]));
}