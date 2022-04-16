use crate::lc::Solution;

/// #pow
impl Solution {
    pub fn largest_palindrome(n: i32) -> i32 {
        if n == 1 {
            return 9;
        }
        let upper = (10 as i32).pow(n as u32) - 1;
        let mut res = 0;
        let mut left = upper;
        while res == 0 {
            let mut p = left as i64;
            let mut x = left;
            while x > 0  {
                p = p * 10 + (x % 10) as i64;
                x /= 10;
            }
            let mut x = upper as i64;
            while x * x > p {
                if p % x == 0 {
                    res = p % 1337;
                    break;
                }
                x -= 1;
            }
            left -= 1;
        }
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(9, Solution::largest_palindrome(1));
    assert_eq!(987, Solution::largest_palindrome(2));
}