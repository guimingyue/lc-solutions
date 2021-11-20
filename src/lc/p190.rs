use crate::lc::Solution;

/// #>>, <<
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut n = x;
        let mut res = n & 1;
        for i in 1..32 {
            n = n >> 1;
            res = (res << 1) + (n & 1);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(0b00111001011110000010100101000000, Solution::reverse_bits(0b00000010100101000001111010011100));
    assert_eq!(0b11111111111111111111111111111101, Solution::reverse_bits(0b10111111111111111111111111111111));
}