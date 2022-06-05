struct Solution {}

/// #Vec
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len() as i32;
        let n = matrix[0].len() as i32;
        let mut left = 0;
        let mut right = m * n - 1;
        while left <= right {
            let mid = (left + right) / 2;
            let midVal = matrix[(mid/n) as usize][(mid%n) as usize];
            if midVal == target {
                return true;
            } else if midVal > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 3));
    assert!(!Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 13));
    assert!(!Solution::search_matrix(vec![vec![5]], 13));
    assert!(Solution::search_matrix(vec![vec![5]], 5));
}