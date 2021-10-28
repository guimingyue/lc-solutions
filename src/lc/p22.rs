use crate::lc::Solution;

/// #Vec, String
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut vec = vec![];
        Solution::generate(n, n, &mut String::new(),&mut vec);
        vec
    }

    fn generate(left: i32, right: i32, buf: &mut String, res: &mut Vec<String>) {
        if left > right {
            return;
        }
        if left == 0 && right == 0 {
            res.push(buf.clone());
            return;
        }
        if left > 0 {
            buf.push('(');
            Solution::generate(left - 1, right, buf, res);
            buf.remove(buf.len()-1);
        }
        if right > 0 {
            buf.push(')');
            Solution::generate(left, right - 1, buf, res);
            buf.remove(buf.len()-1);
        }
    }
}

#[test]
fn test() {
    assert_eq!(vec!["()"], Solution::generate_parenthesis(1));
    assert_eq!(vec!["(())", "()()"], Solution::generate_parenthesis(2));
    assert_eq!(vec!["((()))", "(()())", "(())()", "()(())", "()()()"], Solution::generate_parenthesis(3));
}