use crate::lc::Solution;

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut vec = vec![];
        for i in 0..positions.len() {
            let (left1, right1) = (positions[i][0], positions[i][0] + positions[i][1]);
            let mut height = positions[i][1];
            for j in 0..i {
                let (left2, right2) = (positions[j][0], positions[j][0] + positions[j][1]);
                if left1 < right2 && left2 < right1 {
                    height = height.max(vec[j] + positions[i][1]);
                }
            }
            vec.push(height);
        }
        for i in 1..vec.len() {
            vec[i] = vec[i].max(vec[i-1]);
        }
        vec
    }
}

#[test]
fn test() {
    assert_eq!(vec![2,5,5], Solution::falling_squares(vec![vec![1,2],vec![2,3],vec![6,1]]));
    assert_eq!(vec![100,100], Solution::falling_squares(vec![vec![100,100],vec![200,100]]));
}