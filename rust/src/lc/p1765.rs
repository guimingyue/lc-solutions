use crate::lc::Solution;

use std::collections::vec_deque::VecDeque;

/// #VecDeque
impl Solution {
    pub fn highest_peak(is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const dir: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
        let (m, n) = (is_water.len(), is_water[0].len());
        let mut res = vec![vec![-1; n]; m];
        let mut queue = VecDeque::new();
        for i in 0..m {
            for j in 0..n {
                if is_water[i][j] == 1 {
                    queue.push_back((i, j));
                    res[i][j] = 0;
                }
            }
        }
        while let Some((i, j)) = queue.pop_front() {
            for k in 0..dir.len() {
                let x = dir[k][0] + i as i32;
                let y = dir[k][1] + j as i32;
                if x >= 0 && x < m as i32 && y >= 0 && y < n as i32 && res[x as usize][y as usize] == -1 {
                    res[x as usize][y as usize] = res[i][j] + 1;
                    queue.push_back((x as usize, y as usize));
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    fn assert(param: Vec<Vec<i32>>, expect: i32) {
        let res = Solution::highest_peak(param);
        let mut max = 0;
        for i in 0..res.len() {
            for j in 0..res[0].len() {
                max = max.max(res[i][j]);
            }
        }
        assert_eq!(expect, max);
    }
    assert(vec![vec![0,1],vec![0,0]], 2);
    assert(vec![vec![0,0,1],vec![1,0,0],vec![0,0,0]], 2);
}
