use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let (mut row, mut col) = (vec![], vec![]);
        for i in 0..grid.len() {
            let mut max = grid[i][0];
            for j in 1..grid[i].len() {
                max = max.max(grid[i][j]);
            }
            row.push(max);
        }

        for j in 0..grid[0].len() {
            let mut max = grid[0][j];
            for i in 1..grid.len() {
                max = max.max(grid[i][j]);
            }
            col.push(max);
        }

        let mut res = 0;
        for i in 0..row.len() {
            for j in 0..col.len() {
                let v = std::cmp::min(row[i], col[j]);
                res += v - grid[i][j];
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(35, Solution::max_increase_keeping_skyline(vec![vec![3,0,8,4],vec![2,4,5,7],vec![9,2,6,3],vec![0,3,1,0]]));
}