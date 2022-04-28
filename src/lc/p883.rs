use std::collections::HashMap;
use crate::lc::Solution;

/// #HashMap
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut x = 0;
        let mut x_map: HashMap<usize, i32> = HashMap::new();
        let mut y_map: HashMap<usize, i32> = HashMap::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    continue;
                }
                x += 1;
                let z = grid[i][j];
                match x_map.get_mut(&i) {
                    Some(v) => {*v = std::cmp::max(*v, z);},
                    None => {x_map.insert(i, z);},
                };
                match y_map.get_mut(&j) {
                    Some(v) => {*v = std::cmp::max(*v, z);},
                    None => {y_map.insert(j, z);},
                };
            }
        }
        let y: i32 = x_map.iter().map(|(k, v)| *v as i32).sum();
        let z: i32 = y_map.iter().map(|(k, v)| *v as i32).sum();
        x + y + z
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::projection_area(vec![vec![0]]));
    assert_eq!(17, Solution::projection_area(vec![vec![1, 2], vec![3, 4]]));
    assert_eq!(5, Solution::projection_area(vec![vec![2]]));
    assert_eq!(8, Solution::projection_area(vec![vec![1, 0],vec![0, 2]]));
}