use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn get_maximum_gold(mut grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32, mut cur: i32, res: &mut i32) {
            let bak = grid[i as usize][j as usize];
            cur += bak;
            if cur > *res {
                *res = cur;
            }

            grid[i as usize][j as usize] = 0;
            for d in DIRS {
                let di = i + d.0;
                let dj = j + d.1;
                if di < 0 || dj < 0 || di >= grid.len() as i32 || dj >= grid[0].len() as i32 {
                    continue;
                }
                if grid[di as usize][dj as usize] != 0 {
                    dfs(grid, di, dj, cur, res);
                }
            }
            grid[i as usize][j as usize] = bak;
        }

        let mut res = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                dfs(&mut grid, i as i32, j as i32, 0, &mut res);
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::get_maximum_gold(vec![vec![0]]));
    assert_eq!(1, Solution::get_maximum_gold(vec![vec![1]]));
    assert_eq!(3, Solution::get_maximum_gold(vec![vec![1,2]]));
    assert_eq!(2, Solution::get_maximum_gold(vec![vec![1,0,2]]));
    assert_eq!(24, Solution::get_maximum_gold(vec![vec![0,6,0],vec![5,8,7],vec![0,9,0]]));
    assert_eq!(28, Solution::get_maximum_gold(vec![vec![1,0,7],vec![2,0,6],vec![3,4,5],vec![0,3,0],vec![9,0,20]]));
}