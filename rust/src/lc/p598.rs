use crate::lc::Solution;

/// #Vec, min
impl Solution {
    pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
        let (mut min_m, mut min_n) = (m, n);
        ops.iter().for_each(|vec| {
            min_m = min_m.min(vec[0]);
            min_n = min_n.min(vec[1]);
        });
        min_m * min_n
    }
}

#[test]
fn test() {
    assert_eq!(4, Solution::max_count(3,3,vec![vec![2,2],vec![3,3]]));
    assert_eq!(1, Solution::max_count(3,3,vec![vec![2,2],vec![1,1]]));
    assert_eq!(1, Solution::max_count(3,3,vec![vec![2,2],vec![2,1],vec![1,2]]));
}