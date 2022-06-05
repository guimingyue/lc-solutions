use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();
        let mut res = vec![];
        for i in 0..n {
            let mut k = i;
            let mut j = -1;
            while ((j + 1) as usize) < m  {
                let l = (j + 1) as usize;
                if grid[l][k] == 1 {
                    if k == n - 1 || grid[l][k + 1] == -1{
                        break;
                    } else {
                        k += 1;
                    }
                } else {
                    if k == 0 || grid[l][k-1] == 1 {
                        break;
                    } else {
                        k -= 1;
                    }
                }
                j += 1;
            }
            if ((j + 1) as usize) == m {
                res.push(k as i32);
            } else {
                res.push(-1);
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![1,-1,-1,-1,-1],
               Solution::find_ball(vec![vec![1,1,1,-1,-1],vec![1,1,1,-1,-1],vec![-1,-1,-1,1,1],vec![1,1,1,1,-1],vec![-1,-1,-1,-1,-1]]));
    assert_eq!(vec![-1],
               Solution::find_ball(vec![vec![-1]]));
    assert_eq!(vec![0,1,2,3,4,-1],
               Solution::find_ball(vec![vec![1,1,1,1,1,1],vec![-1,-1,-1,-1,-1,-1],vec![1,1,1,1,1,1],vec![-1,-1,-1,-1,-1,-1]]));
}