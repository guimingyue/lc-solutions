use crate::lc::Solution;

/// #Vec, BinaryHeap, std::cmp::Reverse, std::cmp::max
impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        if height_map.len() <= 2 || height_map[0].len() <= 2 {
            return 0;
        }
        let m = height_map.len();
        let n = height_map[0].len();
        let mut heap = std::collections::BinaryHeap::new();
        let mut vis = Vec::with_capacity(m);
        for i in 0..m {
            vis.push(Vec::with_capacity(n));
            for j in 0..n {
                vis[i].push(false);
            }
        }

        for i in 0..n {
            heap.push(std::cmp::Reverse((height_map[0][i], 0, i)));
            heap.push(std::cmp::Reverse((height_map[m-1][i], m-1, i)));
            vis[0][i] = true;
            vis[m-1][i] = true;
        }
        for i in 0..m {
            heap.push(std::cmp::Reverse((height_map[i][0], i, 0)));
            heap.push(std::cmp::Reverse((height_map[i][n-1], i, n-1)));
            vis[i][0] = true;
            vis[i][n-1] = true;
        }

        let mut res = 0;
        let direction:Vec<Vec<i32>> = vec![vec![0, -1], vec![0, 1],  vec![1, 0], vec![-1, 0]];
        let dr = &direction;
        while !heap.is_empty() {
            let std::cmp::Reverse((h, x, y)) = heap.pop().unwrap();
            for vd in dr {
                let tx = x as i32 + vd[0];
                let ty = y as i32 + vd[1];
                if tx < 0 || tx >= m as i32 || ty < 0 || ty >= n as i32 {
                    continue;
                }
                let nx = tx as usize;
                let ny = ty as usize;
                if vis[nx][ny] {
                    continue;
                }
                if h > height_map[nx][ny] {
                    res += h - height_map[nx][ny];
                }
                let height = std::cmp::max(h, height_map[nx][ny]);
                heap.push(std::cmp::Reverse((height, nx, ny)));
                vis[nx][ny] = true;
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(4, Solution::trap_rain_water(vec![vec![1,4,3,1,3,2],vec![3,2,1,3,2,4],vec![2,3,3,2,3,1]]));
    assert_eq!(10, Solution::trap_rain_water(vec![vec![3,3,3,3,3],vec![3,2,2,2,3],vec![3,2,1,2,3],vec![3,2,2,2,3], vec![3,3,3,3,3]]));
    assert_eq!(14, Solution::trap_rain_water(vec![vec![12,13,1,12],vec![13,4,13,12],vec![13,8,10,12],vec![12,13,12,12],vec![13,13,13,13]]));
}