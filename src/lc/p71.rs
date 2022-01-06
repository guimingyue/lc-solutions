use crate::lc::Solution;

/// #String, Vec
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = vec![];
        let mut dir_begin = false;
        let mut dir_name = String::new();
        for ch in path.chars() {
            if ch == '/' {
                if dir_begin {
                    if dir_name.eq("..") {
                        stack.pop();
                    } else if !dir_name.eq("."){
                        if dir_name.len() > 0 {
                            stack.push(dir_name);
                        }
                    }
                    dir_name = String::new();
                } else {
                    dir_begin = true;
                }
            } else {
                dir_name.push(ch);
            }
        }
        if dir_name.len() > 0 {
            if dir_name.eq("..") {
                stack.pop();
            } else if !dir_name.eq(".") {
                stack.push(dir_name);
            }
        }

        let mut res = String::from("/");
        while stack.len() > 0 {
            res.push_str(stack.remove(0).as_str());
            if stack.len() > 0 {
                res.push_str("/");
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!("/a/b/c", Solution::simplify_path("/a//b////c/d//././/..".to_string()));
    assert_eq!("/home", Solution::simplify_path("/home/".to_string()));
    assert_eq!("/home", Solution::simplify_path("/home".to_string()));
    assert_eq!("/a", Solution::simplify_path("/a/".to_string()));
    assert_eq!("/a", Solution::simplify_path("/a/./".to_string()));
    assert_eq!("/", Solution::simplify_path("/a/..".to_string()));
    assert_eq!("/", Solution::simplify_path("/a/../".to_string()));
    assert_eq!("/home/foo", Solution::simplify_path("/home//foo/".to_string()));
    assert_eq!("/c", Solution::simplify_path("/a/./b/../../c/".to_string()));
}