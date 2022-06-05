use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn color_border(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, row: i32, col: i32, color: i32, boarder: &mut Vec<(usize, usize)>) {
            if visited[row as usize][col as usize] || grid[row as usize][col as usize] != color {
                return;
            }
            if row == 0 || col == 0 || row == (grid.len() - 1) as i32 || col == (grid[0].len() - 1) as i32 {
                boarder.push((row as usize, col as usize));
            }
            const direction:[(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            visited[row as usize][col as usize] = true;
            for (r, c) in direction {
                let next_r = row + r;
                let next_c = col + c;
                if next_r < 0 || next_c < 0 || next_r >= grid.len() as i32 || next_c >= grid[0].len() as i32 {
                    continue;
                }
                if color != grid[next_r as usize][next_c as usize] {
                    boarder.push((row as usize, col as usize));
                }
                if  next_r >= 0 || next_c >= 0 || next_r < grid.len() as i32 || next_c < grid[0].len() as i32 {
                    dfs(grid, visited,  next_r, next_c, color, boarder);
                }
            }
        }
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut boarder = vec![];
        dfs(&grid, &mut visited, row, col, grid[row as usize][col as usize], &mut boarder);
        let mut grid = grid;
        for (r, c) in boarder {
            grid[r][c] = color;
        }
        grid
    }
}

#[test]
fn test() {
    assert_eq!(vec![vec![2,2,2,2,2],vec![2,2,2,2,2],vec![2,2,2,2,2],vec![1,2,2,2,2],vec![2,2,2,2,2]], Solution::color_border(vec![vec![2,1,2,2,1],vec![1,1,1,1,1],vec![2,2,2,1,2],vec![1,2,2,1,2],vec![2,1,1,1,2]], 1, 4, 2));
    assert_eq!(vec![vec![1,3,3], vec![2,3,3]], Solution::color_border(vec![vec![1,2,2], vec![2,3,2]], 0, 1, 3));
    assert_eq!(vec![vec![1,3,3], vec![2,3,3], vec![1,3,3], vec![1,1,1]], Solution::color_border(vec![vec![1,2,2], vec![2,3,2], vec![1,2,2], vec![1,1,1]], 0, 1, 3));
    assert_eq!(vec![vec![2,2,2], vec![2,1,2], vec![2,2,2]], Solution::color_border(vec![vec![1,1,1], vec![1,1,1], vec![1,1,1]], 1, 1, 2));
    assert_eq!(vec![vec![3,3], vec![3,2]], Solution::color_border(vec![vec![1,1], vec![1,2]], 0, 0, 3));
}