use crate::offer::Solution;

impl Solution {
    pub fn sequence_reconstruction(nums: Vec<i32>, sequences: Vec<Vec<i32>>) -> bool {
        let mut graph = vec![vec![]; nums.len() + 1];
        let mut in_depth = vec![0; nums.len() + 1];
        for sequence in &sequences {
            for i in 1..sequence.len() {
                let (x, y) = (sequence[i-1], sequence[i]);
                graph[x as usize].push(y);
                in_depth[y as usize] += 1
            }
        }

        let mut queue = vec![];
        for i in 1..in_depth.len() {
            if in_depth[i] == 0 {
                queue.push(i);
            }
        }

        while queue.len() > 0 {
            if queue.len() > 1 {
                return false;
            }
            let v = queue.remove(0);
            for x in &graph[v] {
                in_depth[*x as usize] -= 1;
                if in_depth[*x as usize] == 0 {
                    queue.push(*x as usize);
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::sequence_reconstruction(vec![1,2,3], vec![vec![1,2], vec![1,3],vec![2,3]]));
    assert!(!Solution::sequence_reconstruction(vec![1,2,3], vec![vec![1,2], vec![1,3]]));
    assert!(!Solution::sequence_reconstruction(vec![1,2,3], vec![vec![1,2]]));
}