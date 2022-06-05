use crate::lc::Solution;

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut res = 0;
        let mut k = 5;
        while k <= n {
            let mut v = k;
            while v % 5 == 0 {
                res += 1;
                v /= 5;
            }
            k += 5;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(0, Solution::trailing_zeroes(3));
    assert_eq!(1, Solution::trailing_zeroes(5));
    assert_eq!(0, Solution::trailing_zeroes(0));
}