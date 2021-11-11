use crate::lc::Solution;

/// #u32, i32
impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let mut num = n;
        let mut count = num & 1;
        let i = 0;
        while num > 0 {
            num = num >> 1;
            count += num & 1;
        }
        count as i32
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::hammingWeight(0b00000000000000000000000000001011));
    assert_eq!(1, Solution::hammingWeight(0b00000000000000000000000010000000));
    assert_eq!(31, Solution::hammingWeight(0b11111111111111111111111111111101));
    assert_eq!(30, Solution::hammingWeight(0b11111111111111111111111111111100));
}