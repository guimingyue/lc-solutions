use crate::lc::Solution;

/// Vec
impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut stack = vec![];
        for s in &ops {
            let mut v = 0;
            if s.len() > 1 {
                v = s.parse().unwrap();
            } else {
                let ch = s.chars().last().unwrap();
                if ch == 'C' {
                    stack.pop();
                    continue;
                } else {
                    v = match ch {
                        'D' => stack[stack.len() - 1] * 2,
                        '+' => stack[stack.len() - 1] + stack[stack.len() - 2],
                        _ => ch as i32 - '0' as i32
                    };
                }
            }
            stack.push(v);
        }
        let mut res = 0;
        for v in stack {
            res += v;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(30, Solution::cal_points(vec_str_to_vec_string(vec!["5","2","C","D","+"])));
    assert_eq!(27, Solution::cal_points(vec_str_to_vec_string(vec!["5","-2","4","C","D","9","+","+"])));
    assert_eq!(1, Solution::cal_points(vec_str_to_vec_string(vec!["1"])));
}

fn vec_str_to_vec_string(strs: Vec<&str>) -> Vec<String> {
    let mut res = vec![];
    for s in strs {
        res.push(s.to_string());
    }
    res
}