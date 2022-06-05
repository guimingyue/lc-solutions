struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = (right - left + 1) / 2 + left;
            let total = (1 + mid as i64) * mid as i64 / 2;
            if total <= n as i64 {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::arrange_coins(5));
    assert_eq!(3, Solution::arrange_coins(8));
    assert_eq!(3, Solution::arrange_coins(6));
    assert_eq!(1, Solution::arrange_coins(1));
    assert_eq!(1, Solution::arrange_coins(2));
    assert_eq!(2, Solution::arrange_coins(3));
    assert_eq!(60070, Solution::arrange_coins(1804289383));
}