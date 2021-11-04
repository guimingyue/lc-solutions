use crate::lc::Solution;

/// #Vec, BinaryHeap, min, max, Reverse, Tuple
impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let (m,n) = (heights.len(), heights[0].len());
        if m <= 1 && n <= 1 {
            return 0;
        }
        let mut vec = vec![1000000; m * n];
        let mut heap:std::collections::BinaryHeap<std::cmp::Reverse<(i32, usize, usize)>> = std::collections::BinaryHeap::new();
        let mut vis = vec![vec![false; m * n]; m * n];
        vec[0] = 0;
        heap.push(std::cmp::Reverse((vec[0], 0, 0)));
        vis[0][0] = true;

        let direction:Vec<(i32, i32)> = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        while let Some(std::cmp::Reverse((h, x, y))) = heap.pop() {
            (0..4).map(|i| (x as i32 + direction[i].0, y as i32 + direction[i].1))
                .filter(|(dx, dy)| *dx >= 0 && *dx < m as i32 && *dy >= 0 && *dy < n as i32)
                .for_each(|(ux, uy)| {
                    let dx = ux as usize;
                    let dy = uy as usize;
                    let pre_idx = x * n + y;
                    let idx = dx * n + dy;
                    if vis[pre_idx][idx] || vis[idx][pre_idx] {
                        return;
                    }
                    let dis = std::cmp::max((heights[dx][dy] - heights[x][y]).abs(), h);
                    vec[idx] = std::cmp::min(dis, vec[idx]);
                    heap.push(std::cmp::Reverse((vec[idx], dx, dy)));
                    vis[pre_idx][idx] = true;
                    vis[idx][pre_idx] = true;
                });
        }
        vec[m * n - 1]
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::minimum_effort_path(vec![vec![3]]));
    assert_eq!(2, Solution::minimum_effort_path(vec![vec![3,1]]));
    assert_eq!(0, Solution::minimum_effort_path(vec![vec![1,1]]));
    assert_eq!(0, Solution::minimum_effort_path(vec![vec![1],vec![1]]));
    assert_eq!(2, Solution::minimum_effort_path(vec![vec![1],vec![3]]));
    assert_eq!(1, Solution::minimum_effort_path(vec![vec![1,2,3],vec![3,8,4],vec![5,3,5]]));
    assert_eq!(2, Solution::minimum_effort_path(vec![vec![1,2,2], vec![3,8,3], vec![5,3,5]]));
    assert_eq!(0, Solution::minimum_effort_path(vec![vec![1,2,1,1,1],vec![1,2,1,2,1],vec![1,2,1,2,1],vec![1,2,1,2,1],vec![1,1,1,2,1]]));
}