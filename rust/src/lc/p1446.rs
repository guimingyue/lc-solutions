use crate::lc::Solution;

/// #String, max
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut res = 0;
        let mut cur = 0;
        let mut pre = ' ';
        for ch in s.chars() {
            if pre != ch {
                res = cur.max(res);
                cur = 1;
                pre = ch;
            } else {
                cur += 1;
            }
        }
        res = cur.max(res);
        res
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::max_power("l".to_string()));
    assert_eq!(2, Solution::max_power("ll".to_string()));
    assert_eq!(1, Solution::max_power("lele".to_string()));
    assert_eq!(2, Solution::max_power("leetcode".to_string()));
    assert_eq!(5, Solution::max_power("abbcccddddeeeeedcba".to_string()));
    assert_eq!(5, Solution::max_power("triplepillooooow".to_string()));
    assert_eq!(11, Solution::max_power("hooraaaaaaaaaaay".to_string()));
    assert_eq!(1, Solution::max_power("tourist".to_string()));
}