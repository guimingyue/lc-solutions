use crate::lc::Solution;

/// #count_ones, ^
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let v = x ^ y;
        v.count_ones() as i32
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::hamming_distance(1, 4));
    assert_eq!(1, Solution::hamming_distance(3, 1));
}