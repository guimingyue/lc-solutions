use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        fn init(is: usize, ie: usize, js: usize, je: usize, grid: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) {
            for i in is..=ie {
                for j in js..=je {
                    if grid[i-1][j-1] == 1 {
                        return;
                    }
                    dp[i][j] = 1;
                }
            }
        }

        let (m_len, n_len) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![vec![0; n_len + 1]; m_len + 1];
        init(1, m_len, 1, 1, &obstacle_grid, &mut dp);
        init(1, 1, 1, n_len, &obstacle_grid, &mut dp);
        for i in 2..=m_len {
            for j in 2..=n_len {
                if obstacle_grid[i-1][j-1] == 1 {
                    dp[i][j] = 0;
                } else {
                   dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }
        dp[m_len][n_len]
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::unique_paths_with_obstacles(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]]));
    assert_eq!(1, Solution::unique_paths_with_obstacles(vec![vec![0,1,0],vec![0,1,0],vec![0,0,0]]));
    assert_eq!(1, Solution::unique_paths_with_obstacles(vec![vec![0,1],vec![0,0]]));
    assert_eq!(1, Solution::unique_paths_with_obstacles(vec![vec![0,0,0]]));
    assert_eq!(0, Solution::unique_paths_with_obstacles(vec![vec![0,1,0]]));
}