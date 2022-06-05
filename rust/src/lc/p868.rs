use crate::lc::Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut v = n;
        while v & 1 == 0 {
            v = v >> 1;
        }
        v = v >> 1;
        let mut shift = 1;
        let mut max = 0;
        while v > 0 {
            if v & 1 == 1 {
                if shift > max {
                    max = shift;
                }
                shift = 0;
            }
            v = v >> 1;
            shift += 1;
        }
        max
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::binary_gap(1000));
    assert_eq!(2, Solution::binary_gap(22));
    assert_eq!(0, Solution::binary_gap(8));
    assert_eq!(2, Solution::binary_gap(5));
}