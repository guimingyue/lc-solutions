use crate::lc::Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut a = vec![0; edges.len() + 2];
        let mut res = 0;
        for edge in edges {
            let e0 = edge[0] as usize;
            a[e0] += 1;
            if a[e0] > 1 {
                res = edge[0];
                break;
            }
            let e1 = edge[1] as usize;
            a[e1] += 1;
            if a[e1] > 1 {
                res = edge[1];
                break;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::find_center(vec![vec![1,2],vec![2,3],vec![4,2]]));
    assert_eq!(1, Solution::find_center(vec![vec![1,2],vec![5,1],vec![1,3],vec![1,4]]));
}