use std::collections::HashMap;
use crate::lc::Solution;

/// #HashMap
impl Solution {
    /// Caution: This solution will be tle
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn dfs(i: i32, map: &HashMap<i32, Vec<i32>>, visited: &mut Vec<bool>, memo: &mut HashMap<(i32, i32), i32>) -> i32 {
            visited[i as usize] = true;
            match map.get(&i) {
                Some(vec) => {
                    let mut max = 0;
                    for v in vec {
                        if visited[*v as usize] {
                            continue;
                        }
                        let mut cur_len = 0;
                        if memo.contains_key(&(i, *v)) {
                            cur_len = *memo.get(&(i, *v)).unwrap();
                        } else {
                            cur_len = dfs(*v, map, visited, memo);
                            memo.insert((i, *v), cur_len);
                        }
                        max = max.max(cur_len);
                    }
                    visited[i as usize] = false;
                    max + 1
                },
                None => 0
            }
        }

        let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
        for e in &edges {
            let v1 = map.entry(e[0]).or_default();
            v1.push(e[1]);
            let v2 = map.entry(e[1]).or_default();
            v2.push(e[0]);
        }
        let mut min = n;
        let mut visited = vec![false; n as usize];
        let mut memo = HashMap::new();
        let mut res = vec![];
        for i in 0..n {
            let len = dfs(i, &map, &mut visited, &mut memo);
            if len < min {
                res.clear();
                res.push(i);
                min = len;
            } else if len == min {
                res.push(i);
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![1], Solution::find_min_height_trees(4, vec![vec![1,0],vec![1,2],vec![1,3]]));
    assert_eq!(vec![3,4], Solution::find_min_height_trees(6, vec![vec![3,0],vec![3,1],vec![3,2],vec![3,4],vec![5,4]]));
    assert_eq!(vec![3,4], Solution::find_min_height_trees(6, vec![vec![0,3],vec![1,3],vec![3,2],vec![3,4],vec![5,4]]));
}