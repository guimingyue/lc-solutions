use crate::lc::Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        while i < bits.len() {
            if i == bits.len() - 1 {
                return true;
            }
            if bits[i] == 1 {
                i += 1;
            }
            i += 1;
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::is_one_bit_character(vec![1, 0, 0]));
    assert!(!Solution::is_one_bit_character(vec![1, 1, 1, 0]));
}