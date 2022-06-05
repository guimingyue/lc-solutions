use crate::offer::Solution;

/// #abs
/// https://leetcode-cn.com/problems/xoh6Oh/
impl Solution {
    pub fn divide(a: i32, b: i32) -> i32 {
        if a == i32::MIN && b == -1 {
            return i32::MAX;
        }
        let mut flag = 1;
        if (a > 0 && b < 0) || (a < 0 && b > 0) {
            flag = -1;
        }
        let mut a = (a as i64).abs();
        let mut b = (b as i64).abs();
        let mut res = 0;
        let mut temp:i64 = 1;
        let base_b = b;
        while a >= b {
            let temp_b = b + b;
            if a < temp_b {
                res += temp;
                temp = 1;
                a -= b;
                b = base_b;
                continue
            }
            temp = temp << 1;
            b = temp_b;

        }
        if flag == -1 {
            res = -res;
        }
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(-2147483648, Solution::divide(-2147483648, 1));
    assert_eq!(-1073741823, Solution::divide(2147483647, -2));
    assert_eq!(7, Solution::divide(15, 2));
    assert_eq!(-2, Solution::divide(7, -3));
    assert_eq!(0, Solution::divide(0, 1));
    assert_eq!(1, Solution::divide(1, 1));
    assert_eq!(2, Solution::divide(2, 1));
    assert_eq!(-2, Solution::divide(2, -1));
    assert_eq!(0, Solution::divide(1, 2));
    assert_eq!(0, Solution::divide(1, -2));
}