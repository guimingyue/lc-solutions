use crate::lc::Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let v = (n >> 1) ^ n;
        v & (v+1) == 0
    }
}

#[test]
fn test() {
    assert!(Solution::has_alternating_bits(5));
    assert!(!Solution::has_alternating_bits(7));
    assert!(!Solution::has_alternating_bits(11));
    assert!(!Solution::has_alternating_bits(9));
    assert!(Solution::has_alternating_bits(1));
    assert!(Solution::has_alternating_bits(2));
    assert!(!Solution::has_alternating_bits(3));
}