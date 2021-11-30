use crate::lc::Solution;

/// #pow
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut count = 0;
        let mut len = 1;
        let base: i64 = 10;
        let nine_base: i64 = 9;
        loop {
            let next = (nine_base * len as i64 * base.pow((len-1) as u32)) as i64;
            if count + next > n as i64{
                break
            }
            count += next;
            len += 1;
        }

        let num = (n - count as i32) / len;
        let mut num_mode = (n - count as i32) % len;

        let mut val = num;
        if len - 1 > 0 {
            val += (base.pow((len-1) as u32) - 1) as i32
        }

        let mut idx = 0;
        if num_mode > 0 {
            val += 1;
            idx = len - num_mode;
        }

        while idx > 0 {
            val /= 10;
            idx -= 1;
        }

        val % 10
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::find_nth_digit(3));
    assert_eq!(0, Solution::find_nth_digit(11));
    assert_eq!(1, Solution::find_nth_digit(12));
    assert_eq!(1, Solution::find_nth_digit(13));
    assert_eq!(1, Solution::find_nth_digit(14));
    assert_eq!(2, Solution::find_nth_digit(15));
    assert_eq!(0, Solution::find_nth_digit(51));
    assert_eq!(3, Solution::find_nth_digit(52));
    assert_eq!(1, Solution::find_nth_digit(190));
    assert_eq!(0, Solution::find_nth_digit(191));
    assert_eq!(0, Solution::find_nth_digit(192));
    assert_eq!(1, Solution::find_nth_digit(193));
    assert_eq!(0, Solution::find_nth_digit(194));
    assert_eq!(2, Solution::find_nth_digit(2147483647));
}