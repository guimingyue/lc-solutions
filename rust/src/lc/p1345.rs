use std::collections::{HashMap, HashSet, VecDeque};
use crate::lc::Solution;

/// #HashMap, HashSet, VecDeque
impl Solution {
    pub fn min_jumps(arr: Vec<i32>) -> i32 {
        let mut map = HashMap::<i32, Vec<usize>>::new();
        for i in 0..arr.len() {
            let entry = map.entry(arr[i]).or_default();
            entry.push(i);
        }
        let mut queue = VecDeque::<(usize, i32)>::new();
        queue.push_back((0, 0));
        let mut visited = HashSet::new();
        visited.insert(0);
        while let Some((idx, step)) = queue.pop_front() {
            visited.insert(idx);
            if idx == arr.len() -1 {
                return step;
            }
            if map.contains_key(&arr[idx]) {
                let idx_list = map.remove(&arr[idx]).unwrap();
                for v in idx_list {
                    queue.push_back((v, step + 1));
                }
            }
            if idx + 1 < arr.len() && visited.insert(idx + 1) {
                queue.push_back((idx + 1, step + 1));
            }
            if idx >= 1 && visited.insert(idx - 1) {
                queue.push_back((idx - 1, step + 1));
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::min_jumps(vec![100,-23,-23,404,100,23,23,23,3,404]));
    assert_eq!(0, Solution::min_jumps(vec![7]));
    assert_eq!(1, Solution::min_jumps(vec![7,6,9,6,9,6,9,7]));
    assert_eq!(2, Solution::min_jumps(vec![6,1,9]));
    assert_eq!(3, Solution::min_jumps(vec![11,22,7,7,7,7,7,7,7,22,13]));
    assert_eq!(3, Solution::min_jumps(vec![-1, 2, 2, -1, -3, -3]));
}