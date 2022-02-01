use crate::lc::Solution;

/// String, Slice
impl Solution {
    pub fn longest_nice_substring(s: String) -> String {
        fn dfs(s: &str, res: &mut String) {
            if s.len() < 2 {
                return;
            }
            let (mut lower, mut upper) = (0 as u32, 0 as u32);
            for ch in s.chars() {
                if ch.is_lowercase() {
                    lower |= 1 << ch as u8 - 'a' as u8;
                } else {
                    upper |= 1 << ch as u8 - 'A' as u8;
                }
            }
            if lower == upper {
                if s.len() > res.len() {
                    res.clear();
                    res.push_str(s);
                }
                return;
            }
            let valid = lower & upper;
            let mut pos = 0;
            let mut dfs_start = pos;

            for ch in s.chars() {
                if (valid & 1 << (ch.to_ascii_lowercase() as u32 - 'a' as u32)) != 0 {
                    pos += 1;
                    continue;
                }
                dfs(&s[dfs_start..pos], res);
                pos += 1;
                dfs_start = pos;
            }
            dfs(&s[dfs_start..pos], res);
        }

        let mut res = String::new();
        dfs(s.as_str(), &mut res);
        res
    }
}

#[test]
fn test() {
    assert_eq!("aA", Solution::longest_nice_substring("aA".to_string()));
    assert_eq!("Bb", Solution::longest_nice_substring("Bb".to_string()));
    assert_eq!("aAa", Solution::longest_nice_substring("YazaAay".to_string()));
    assert_eq!("YaAay", Solution::longest_nice_substring("YazYaAay".to_string()));
    assert_eq!("", Solution::longest_nice_substring("c".to_string()));
    assert_eq!("dD", Solution::longest_nice_substring("dDzeE".to_string()));
}