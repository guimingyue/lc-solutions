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