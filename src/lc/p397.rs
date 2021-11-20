use crate::lc::Solution;

/// #min
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        fn dfs(n: usize, curr_num: i32, num: &mut i32) {
            if n == 1 {
                *num = std::cmp::min(curr_num, *num);
                return;
            }
            if n & 1 == 1 {
                dfs(n+1, curr_num+1, num);
                dfs(n-1, curr_num+1, num);
            } else {
                dfs(n / 2, curr_num + 1, num);
            }
        }
        let mut res = i32::MAX;
        dfs(n as usize, 0, &mut res);
        res
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::integer_replacement(8));
    assert_eq!(4, Solution::integer_replacement(7));
    assert_eq!(2, Solution::integer_replacement(4));
    assert_eq!(0, Solution::integer_replacement(1));
    assert_eq!(32, Solution::integer_replacement(i32::MAX));
}