use crate::lc::Solution;

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        fn get_steps(cur: i32, n: i32) -> i32 {
            let mut first = cur as i64;
            let mut last = first;
            let mut num = 0;
            while first <= n as i64 {
                num += (last - first + 1) as i32;
                first = first * 10;
                last = std::cmp::min(n as i64, last * 10 + 9);
            }
            num as i32
        }
        let mut cur = 1;
        let mut k = k - 1;
        while k > 0 {
            let m = get_steps(cur, n);
            if k < m {
                cur *= 10;
                k -= 1;
            } else {
                cur += 1;
                k -= m;
            }
        }
        cur
    }
}

#[test]
fn test() {
    assert_eq!(416126219, Solution::find_kth_number(681692778, 351251360));
    assert_eq!(10, Solution::find_kth_number(13, 2));
    assert_eq!(9, Solution::find_kth_number(100, 90));
    assert_eq!(2, Solution::find_kth_number(10, 3));
    assert_eq!(11, Solution::find_kth_number(13, 3));
    assert_eq!(1, Solution::find_kth_number(1, 1));
}