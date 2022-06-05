use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let mut graph = vec![vec![false; quiet.len()]; quiet.len()];
        let mut in_degree = vec![0; quiet.len()];
        for r in richer {
            graph[r[0] as usize][r[1] as usize] = true;
            in_degree[r[1] as usize] += 1;
        }

        let mut queue = vec![];
        for i in 0..in_degree.len() {
            if in_degree[i] == 0 {
                queue.push(i);
            }
        }
        let mut res = vec![];
        for i in 0..quiet.len() {
            res.push(i as i32);
        }

        while let Some(v) = queue.pop() {
            for i in 0..graph[v].len() {
                if !graph[v][i] {
                    continue;
                }
                in_degree[i] -= 1;
                if quiet[res[v] as usize] < quiet[res[i] as usize] {
                    res[i] = res[v];
                }

                if in_degree[i] == 0 {
                    queue.push(i);
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![5,5,2,5,4,5,6,7], Solution::loud_and_rich(vec![vec![1,0],vec![2,1],vec![3,1],vec![3,7],vec![4,3],vec![5,3],vec![6,3]], vec![3,2,5,4,6,1,7,0]));
    assert_eq!(vec![0], Solution::loud_and_rich(vec![], vec![0]));
}