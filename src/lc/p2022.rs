use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if original.len() as i32 != m * n {
            return vec![];
        }
        let mut res = vec![];
        let mut i = 0;
        while i < original.len() {
            let mut line = vec![];
            for j in 0..n {
                line.push(original[i]);
                i += 1;
            }
            res.push(line);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![vec![1],vec![2],vec![3],vec![4]], Solution::construct2_d_array(vec![1,2,3,4], 4, 1));
    assert_eq!(vec![vec![1,2],vec![3,4]], Solution::construct2_d_array(vec![1,2,3,4], 2, 2));
    assert_eq!(vec![vec![1,2,3]], Solution::construct2_d_array(vec![1,2,3], 1, 3));
    assert_eq!(Vec::<Vec<i32>>::new(), Solution::construct2_d_array(vec![1,2], 1, 1));
    assert_eq!(Vec::<Vec<i32>>::new(), Solution::construct2_d_array(vec![3], 1, 2));
}