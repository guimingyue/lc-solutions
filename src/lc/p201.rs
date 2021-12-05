use crate::lc::Solution;

/// #>>, <<, !
impl Solution {

    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut m = left;
        let mut n = right;
        while m < n {
            n = n & (n-1);
        }
        n
    }

    pub fn range_bitwise_and_shift(left: i32, right: i32) -> i32 {
        let mut shift = 0;
        let mut m = left;
        let mut n = right;
        while m < n {
            m = m >> 1;
            n = n >> 1;
            shift += 1;
        }
        n << shift
    }

    pub fn range_bitwise_and_tle(left: i32, right: i32) -> i32 {
        let mut res = i32::MAX;
        for v in left..=right {
            res &= v;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::range_bitwise_and(2, 3));
    assert_eq!(0b1110, Solution::range_bitwise_and(0b1110, 0b1111));
    assert_eq!(12, Solution::range_bitwise_and(12, 13));
    assert_eq!(0, Solution::range_bitwise_and(1, 2));
    assert_eq!(0, Solution::range_bitwise_and(3, 4));
    assert_eq!(4, Solution::range_bitwise_and(4, 5));
    assert_eq!(4, Solution::range_bitwise_and(5, 6));
    assert_eq!(5, Solution::range_bitwise_and(5, 5));
    assert_eq!(4, Solution::range_bitwise_and(5, 7));
    assert_eq!(0, Solution::range_bitwise_and(5, 8));
    assert_eq!(0, Solution::range_bitwise_and(5, 9));
    assert_eq!(0, Solution::range_bitwise_and(0, 0));
    assert_eq!(0, Solution::range_bitwise_and(1, 2147483647));
}