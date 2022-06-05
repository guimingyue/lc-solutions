use crate::lc::Solution;

/// #pow
impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        fn dfs(n: i32, pow: u32) -> bool {
            let v_pow = (3 as i32).pow(pow);
            if n < v_pow {
                return false;
            }
            if n == v_pow {
                return true;
            }
            if dfs(n - v_pow, pow + 1) {
                return true;
            }
            dfs(n, pow + 1)
        }
        dfs(n, 0)
    }
}

#[test]
fn test() {
    assert!(Solution::check_powers_of_three(1));
    assert!(Solution::check_powers_of_three(3));
    assert!(Solution::check_powers_of_three(4));
    assert!(!Solution::check_powers_of_three(5));
    assert!(Solution::check_powers_of_three(12));
    assert!(Solution::check_powers_of_three(91));
    assert!(!Solution::check_powers_of_three(21));
}