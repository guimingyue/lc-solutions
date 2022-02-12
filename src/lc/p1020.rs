use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        const DIRECTION:[(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: i32, j: i32) {
            if i < 0 || i as usize >= grid.len() || j < 0 || j as usize >= grid[0].len() {
                return;
            }
            let (i, j) = (i as usize, j as usize);
            if grid[i][j] == 0 || visited[i][j] {
                return;
            }
            visited[i][j] = true;
            for (x, y) in DIRECTION {
                dfs(grid, visited,i as i32 + x,j as i32 + y);
            }
        }

        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            dfs(&grid, &mut visited, i as i32, 0);
            dfs(&grid, &mut visited, i as i32, (grid[0].len() - 1) as i32);
        }
        for j in 0..grid[0].len() {
            dfs(&grid, &mut visited,0, j as i32);
            dfs(&grid, &mut visited,(grid.len() - 1) as i32, j as i32);
        }
        let mut res = 0;
        for i in 1..grid.len() - 1 {
            for j in 1..grid[0].len() - 1 {
                if grid[i][j] == 1 && !visited[i][j] {
                    res += 1;
                }

            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::num_enclaves(vec![vec![0,0,0,0],vec![1,0,1,0],vec![0,1,1,0],vec![0,0,0,0]]));
    assert_eq!(0, Solution::num_enclaves(vec![vec![0,1,1,0],vec![0,0,1,0],vec![0,0,1,0],vec![0,0,0,0]]));
    assert_eq!(0, Solution::num_enclaves(vec![vec![0],vec![1],vec![1],vec![0],vec![0]]));
    assert_eq!(7, Solution::num_enclaves(vec![vec![0,0,1,1,1,0,1,1,1,0,1],vec![1,1,1,1,0,1,0,1,1,0,0],vec![0,1,0,1,1,0,0,0,0,1,0],vec![1,0,1,1,1,1,1,0,0,0,1],vec![0,0,1,0,1,1,0,0,1,0,0],vec![1,0,0,1,1,1,0,0,0,1,1],vec![0,1,0,1,1,0,0,0,1,0,0],vec![0,1,1,0,1,0,1,1,1,0,0],vec![1,1,0,1,1,1,0,0,0,0,0],vec![1,0,1,1,0,0,0,1,0,0,1]]));
}