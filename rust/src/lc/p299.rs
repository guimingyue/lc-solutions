use crate::lc::Solution;

/// #HashMap, BTreeSet, format!
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut map = Solution::build_map(secret.as_str());
        let g_map = Solution::build_map(guess.as_str());
        let (mut a, mut b) = (0, 0);
        for (k, v) in g_map {
            if let Some(mut set) = map.remove(&k) {
                let mut a_num = 0;
                let mut b_num = 0;
                let len = set.len();
                for v_idx in v {
                    if set.remove(&v_idx) {
                        a_num += 1;
                    } else {
                        b_num += 1;
                    }
                }
                a += a_num;
                b += std::cmp::min(b_num, len - a_num);
            }
        }
        format!("{}A{}B", a, b)
    }

    fn build_map(str: &str) -> std::collections::HashMap<char, std::collections::BTreeSet<usize>> {
        let mut map:std::collections::HashMap<char, std::collections::BTreeSet<usize>> = std::collections::HashMap::new();
        for (idx, v) in str.char_indices() {
            if let Some(set) = map.get_mut(&v) {
                set.insert(idx);
            } else {
                let mut set = std::collections::BTreeSet::new();
                set.insert(idx);
                map.insert(v, set);
            }
        }
        return map;
    }
}

#[test]
fn test() {
    assert_eq!("1A3B", Solution::get_hint("1807".to_string(), "7810".to_string()));
    assert_eq!("1A1B", Solution::get_hint("1123".to_string(), "0111".to_string()));
    assert_eq!("0A0B", Solution::get_hint("1".to_string(), "0".to_string()));
    assert_eq!("1A0B", Solution::get_hint("1".to_string(), "1".to_string()));
    assert_eq!("1A0B", Solution::get_hint("0".to_string(), "0".to_string()));
}