use crate::lc::Solution;

/// #Vec(stack)
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut stack = vec![];
        let mut res = 0;
        for ch in s.chars() {
            if ch == '(' {
                stack.push('(');
                if stack.len() > res {
                    res = stack.len();
                }
            } else if ch == ')' {
                stack.pop();
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(3, Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()));
    assert_eq!(3, Solution::max_depth("(1)+((2))+(((3)))".to_string()));
    assert_eq!(1, Solution::max_depth("1+(2*3)/(2-1)".to_string()));
    assert_eq!(0, Solution::max_depth("1".to_string()));
}