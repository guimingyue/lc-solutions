struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 1 {
            return dividend;
        }
        if divisor == -1 {
            if dividend == i32::MAX {
                return -dividend;
            } else if dividend == i32::MIN {
                return i32::MAX;
            }
        }

        let mut dend = dividend;
        let mut visor = divisor;
        let mut sign = false;
        if dend > 0 {
            dend = -dend;
            sign = !sign;
        }
        if visor > 0 {
            visor = -visor;
            sign = !sign;
        }

        let mut ans = 0;
        while dend <= visor {
            let mut c = visor;
            let mut d = -1;
            while c >= i32::MIN / 2 && c > dend - c {
                c += c;
                d += d;
            }
            dend -= c;
            ans += d;
        }

        if sign {
            ans
        } else {
            -ans
        }
    }
}


#[test]
fn test() {
    assert_eq!(i32::MAX, Solution::divide(i32::MIN, -1));
    assert_eq!(i32::MIN, Solution::divide(i32::MIN, 1));
    assert_eq!(i32::MAX, Solution::divide(i32::MAX, 1));
    assert_eq!(-i32::MAX, Solution::divide(i32::MAX, -1));
    assert_eq!(3, Solution::divide(10, 3));
    assert_eq!(-2, Solution::divide(7, -3));
}