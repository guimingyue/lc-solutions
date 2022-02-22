use crate::lc::Solution;

/// #String
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        fn push_ch(res: &mut String, ch: char, mut len: usize) {
            while len > 0 {
                res.push(ch);
                len -= 1;
            }
        }
        if dominoes.len() == 1 {
            return dominoes;
        }
        let mut res = String::new();
        let bytes = dominoes.into_bytes();
        let mut last = (0, bytes[0] as char);
        if last.1 != '.' {
            res.push(last.1);
        }
        for i in 1..bytes.len() {
            let ch = bytes[i] as char;
            if ch == 'R' {
                if last.1 == 'R' {
                    push_ch(&mut res, 'R', i - last.0 - 1);
                } else if last.1 == 'L' {
                    push_ch(&mut res, '.', i - last.0 - 1);
                } else {
                    push_ch(&mut res, '.', i - last.0);
                }
                res.push(ch);
                last = (i, 'R');
            } else if ch == 'L' {
                if last.1 == 'R' {
                    let side_len = (i - last.0 - 1) / 2;
                    push_ch(&mut res, 'R', side_len);
                    if (i - last.0 - 1) % 2 != 0 {
                        push_ch(&mut res, '.', 1);
                    }
                    push_ch(&mut res, 'L', side_len);
                } else  {
                    push_ch(&mut res, 'L', i - last.0 - 1);
                    if last.1 == '.' {
                        push_ch(&mut res, 'L', 1);
                    }
                }
                res.push(ch);
                last = (i, 'L');
            }
        }
        let last_ch = bytes[bytes.len() - 1] as char;
        if last_ch == '.' {
            if last.1 == 'L' {
                push_ch(&mut res, '.', bytes.len() - last.0 - 1);
            } else if last.1 == 'R' {
                push_ch(&mut res, 'R', bytes.len() - last.0 - 1);
            } else {
                return String::from_utf8(bytes).unwrap();
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!("RR.L", Solution::push_dominoes("RR.L".to_string()));
    assert_eq!("LL.R", Solution::push_dominoes(".L.R".to_string()));
    assert_eq!("LL.RR.LLRRLL..", Solution::push_dominoes(".L.R...LR..L..".to_string()));
    assert_eq!("....", Solution::push_dominoes("....".to_string()));
    assert_eq!("L", Solution::push_dominoes("L".to_string()));
    assert_eq!("R", Solution::push_dominoes("R".to_string()));
    assert_eq!("RL", Solution::push_dominoes("RL".to_string()));
    assert_eq!("LR", Solution::push_dominoes("LR".to_string()));
}