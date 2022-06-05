use crate::lc::Solution;

/// #String
impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut queue = vec![];
        for i in 0..26 {
            let ch = char::from('a' as u8 + i);
            queue.push(ch);
        }

        let mut res = String::new();
        let bytes = s.as_bytes();
        let mark = '?' as u8;

        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == mark {
                let mut j = i + 1;
                while j < bytes.len() && bytes[j] == mark {
                    j += 1;
                }

                let mut ch1 = 'a' as u8 - 1;
                let mut ch2 = 'a' as u8 - 1;
                if i != 0 {
                    ch1 = bytes[i-1];
                }
                if j < bytes.len() && bytes[j] != mark {
                    ch2 = bytes[j];
                }

                for k in i..j {
                    let mut ch = *queue.first().unwrap() as u8;
                    while ch == ch1 || ch == ch2 {
                        queue.remove(0);
                        queue.push(char::from(ch));
                        ch = *queue.first().unwrap() as u8;
                    }
                    res.push(char::from(ch));
                    ch1 = ch;
                }
                i = j;
            } else {
                res.push(char::from(bytes[i]));
                i += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!("azs".to_string(), Solution::modify_string("?zs".to_string()));
    assert_eq!("jaqgacb".to_string(), Solution::modify_string("j?qg??b".to_string()));
}