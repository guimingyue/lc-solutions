struct Solution {}

/// #Vec
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut x:i32 = 0;
        let mut y = matrix[0].len() as i32 - 1;
        while (x as usize) < matrix.len() && y >= 0 {
            let val = matrix[x as usize][y as usize];
            if target == val {
                return true;
            } else if target < val {
                y -= 1;
            } else {
                x += 1;
            }
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::search_matrix(vec![vec![1,4,7,11,15], vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]], 5));
    assert!(!Solution::search_matrix(vec![vec![1,4,7,11,15], vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]], 20));
    assert!(!Solution::search_matrix(vec![vec![5]], 13));
    assert!(Solution::search_matrix(vec![vec![5]], 5));
}