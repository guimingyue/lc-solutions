use std::collections::{BTreeMap, HashMap, VecDeque};
use std::iter::FromIterator;
use crate::lc::Solution;

/// #BTreeMap, HashMap, VecDeque
impl Solution {
    pub fn count_highest_score_nodes(parents: Vec<i32>) -> i32 {
        let n = parents.len();
        let mut vec: Vec<Vec<usize>> = vec![vec![]; parents.len()];
        for i in 0..n {
            if parents[i] == -1 {
                continue;
            }
            vec[parents[i] as usize].push(i);
        }

        fn count_nodes(tree: &Vec<Vec<usize>>, idx: usize, node_count: &mut HashMap<usize, usize>) -> i32 {
            let mut count = 1;
            if tree[idx].len() > 0 {
                for v in &tree[idx] {
                    count += count_nodes(tree, *v, node_count);
                }
            }
            node_count.insert(idx, count as usize);
            count
        }

        let mut node_count = HashMap::new();
        count_nodes(&vec, 0, &mut node_count);

        let mut res:BTreeMap<usize, usize> = BTreeMap::new();
        for i in 0..n {
            let mut score = 1;
            let mut sub_node_count = 1;
            for v in &vec[i] {
                score *= node_count[v];
                sub_node_count += node_count[v];
            }
            if parents[i] >= 0 {
                score *= n - sub_node_count;
            }

            let count = res.entry(score).or_default();
            *count += 1;
        }
        let seq_queue = VecDeque::from_iter(res);
        let back = seq_queue.back().unwrap();
        back.1 as i32
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::count_highest_score_nodes(vec![-1,2,0,2,0]));
    assert_eq!(2, Solution::count_highest_score_nodes(vec![-1,2,0]));
}