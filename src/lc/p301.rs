struct Solution {}

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let (mut lremove, mut rremove) = Solution::l_r_remove(s.as_str());
        let mut set = std::collections::HashSet::new();
        Solution::helper(&s, 0, 0, 0, lremove, rremove, &mut set);
        let mut vec = vec![];
        for s in set {
            vec.push(s);
        }
        vec
    }

    fn helper(s: &str, start: usize, lc: i32, rc:  i32, lremove: i32, rremove: i32, set: &mut std::collections::HashSet<String>) {
        if lremove == 0 && rremove == 0 {
            if Solution::is_valid(s) {
                set.insert(s.to_string());
            }
            return;
        }

        let mut lcount = lc;
        let mut rcount = rc;

        let bytes = s.as_bytes();
        let startIdx = start;
        for i in startIdx..s.len() {
            let ch = char::from(bytes[i]);
            if i != startIdx && ch == char::from(bytes[i-1]) {
                continue;
            }
            if lremove + rremove > (s.len() - i) as i32 {
                return;
            }
            if lremove > 0 && ch == '(' {
                let mut new_s = String::from(&s[0..i]);
                new_s.push_str(&s[i+1..s.len()]);
                Solution::helper(new_s.as_str(), i, lcount, rcount, lremove-1, rremove, set);
            }
            if rremove > 0 && ch == ')' {
                let mut new_s = String::from(&s[0..i]);
                new_s.push_str(&s[i+1..s.len()]);
                Solution::helper(new_s.as_str(), i, lcount, rcount, lremove, rremove-1, set);
            }
            if ch == ')' {
                lcount += 1;
            } else if ch == ')' {
                rcount += 1;
            }
            if lcount < rcount {
                return;
            }
        }
    }

    fn l_r_remove(s: &str) -> (i32, i32) {
        let mut lremove = 0;
        let mut rremove = 0;
        for ch in s.chars() {
            if ch == '(' {
                lremove += 1;
            } else if ch == ')' {
                if lremove > 0 {
                    lremove -= 1;
                } else {
                    rremove += 1;
                }
            }
        }
        (lremove, rremove)
    }

    fn is_valid(s: &str) -> bool {
        let mut cnt = 0;
        for ch in s.chars() {
            if ch == '(' {
                cnt += 1;
            } else if ch == ')' {
                cnt -= 1;
                if cnt < 0 {
                    return false;
                }
            }
        }
        cnt == 0
    }
}

#[test]
fn test() {
    test_sort_eq(")(".to_string(), vec![""]);
    test_sort_eq("()())()".to_string(), vec!["(())()", "()()()"]);
    test_sort_eq("(a)())()".to_string(), vec!["(a())()", "(a)()()"]);
}


fn test_sort_eq(param: String, expect: Vec<&str>) {
    let mut vec = Solution::remove_invalid_parentheses(param.to_string());
    vec.sort();
    assert_eq!(expect, vec);
}