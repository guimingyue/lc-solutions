use crate::lc::Solution;
use std::collections::VecDeque;

/// #VecDeque
impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut graph = vec![vec![]; patience.len()];
        for v in &edges {
            graph[v[0] as usize].push(v[1]);
            graph[v[1] as usize].push(v[0]);
        }
        let mut queue = VecDeque::new();
        let mut visited = vec![false; patience.len()];
        queue.push_back(0);
        visited[0] = true;
        let mut dist = 1;
        while !queue.is_empty() {
            let mut size = queue.len();
            while size > 0 {
                size -= 1;
                let cur = queue.pop_front().unwrap();
                for idx in &graph[cur] {
                    let v = *idx as usize;
                    if visited[v] {
                        continue;
                    }
                    visited[v] = true;
                    queue.push_back(v);
                    let time = dist * 2 + 1 + patience[v] * ((2 * dist - 1) / patience[v]);
                    max = max.max(time);
                }
            }
            dist += 1;
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(8, Solution::network_becomes_idle(vec![vec![0,1],vec![1,2]], vec![0,2,1]));
    assert_eq!(3, Solution::network_becomes_idle(vec![vec![0,1],vec![1,2],vec![0,2]], vec![0,10,10]));
    assert_eq!(3, Solution::network_becomes_idle(vec![vec![0,1],vec![0,2],vec![1,2]], vec![0,10,10]));
}