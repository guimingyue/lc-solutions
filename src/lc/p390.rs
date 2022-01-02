use crate::lc::Solution;

/// #match
impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let (mut a1, mut an) = (1, n);
        let (mut step, mut cnt) = (1, n);
        let mut k = 1;
        while cnt > 1 {
            if k % 2 != 0 {
                a1 = a1 + step;
                an = match cnt % 2 {
                    1 => an - step,
                    _ => an
                };
            } else {
                a1 = match cnt % 2 {
                    1 => a1 + step,
                    _ => a1
                };
                an = an - step;
            }
            k += 1;
            step = step << 1;
            cnt = cnt / 2;
        }
        an
    }
}

#[test]
fn test() {
    assert_eq!(6, Solution::last_remaining(9));
    assert_eq!(1, Solution::last_remaining(1));
    assert_eq!(8, Solution::last_remaining(10));
}