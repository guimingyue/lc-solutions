use crate::lc::Solution;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
   Int(i32),
   List(Vec<NestedInteger>)
}

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        fn parse_num(str: &[u8], idx: &mut usize) -> NestedInteger {
            let mut res = 0;
            let mut flag = 1;
            while *idx < str.len() && str[*idx] != ',' as u8 && str[*idx] != ']' as u8 {
                if str[*idx] == '-' as u8 {
                    flag = -1;
                    *idx += 1;
                    continue;
                }
                res = res * 10 + (str[*idx] - '0' as u8) as i32;
                *idx += 1;
            }
            NestedInteger::Int(res * flag)
        }

        fn parse(str: &[u8], idx: &mut usize) -> NestedInteger {
            if str[*idx] as char == '[' {
                let mut vec = vec![];
                *idx += 1;
                while str[*idx] as char != ']' {
                    if str[*idx] as char == ',' {
                        *idx += 1;
                        continue;
                    } else if str[*idx] as char == '[' {
                        let sub = parse(str, idx);
                        vec.push(sub);
                    } else {
                        vec.push(parse_num(str, idx));
                    }
                }
                *idx += 1;
                NestedInteger::List(vec)
            } else {
                parse_num(str, idx)
            }

        }
        let mut idx = 0;
        parse(s.as_bytes(), &mut idx)
    }
}

#[test]
fn test() {
    assert_eq!(NestedInteger::List(vec![NestedInteger::Int(123), NestedInteger::List(vec![NestedInteger::Int(456)]), NestedInteger::Int(-1)]),
               Solution::deserialize("[123,[456],-1]".to_string()));
    assert_eq!(NestedInteger::List(vec![NestedInteger::Int(123), NestedInteger::List(vec![NestedInteger::Int(456), NestedInteger::List(vec![NestedInteger::Int(789)])])]),
               Solution::deserialize("[123,[456,[789]]]".to_string()));
    assert_eq!(NestedInteger::List(vec![NestedInteger::Int(328), NestedInteger::Int(-28)]), Solution::deserialize("[328,-28]".to_string()));
    assert_eq!(NestedInteger::Int(328), Solution::deserialize("328".to_string()));
    assert_eq!(NestedInteger::Int(-328), Solution::deserialize("-328".to_string()));
    assert_eq!(NestedInteger::List(vec![NestedInteger::Int(-328)]), Solution::deserialize("[-328]".to_string()));
}