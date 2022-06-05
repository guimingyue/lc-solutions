use crate::lc::Solution;

/// #HashMap, Vec
impl Solution {

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut res = vec![];
        if s.len() < p.len() {
            return res;
        }

        let s_bytes = s.into_bytes();
        let p_bytes = p.into_bytes();
        let mut s_count = [0; 26];
        let mut p_count = [0; 26];
        let base = 'a' as u8;
        for i in 0..p_bytes.len() {
            p_count[(p_bytes[i] - base) as usize] += 1;
            s_count[(s_bytes[i] - base) as usize] += 1;
        }
        if s_count.eq(&p_count) {
            res.push(0);
        }
        for i in 0..s_bytes.len() - p_bytes.len() {
            s_count[(s_bytes[i] - base) as usize] -= 1;
            s_count[(s_bytes[i+p_bytes.len()] - base) as usize] += 1;
            if s_count.eq(&p_count) {
                res.push((i+1) as i32);
            }
        }

        res
    }
    /// time limit exceed
    pub fn find_anagrams_tle(s: String, p: String) -> Vec<i32> {
        fn is_match(s: &str, mut map: std::collections::HashMap<char, i32>) -> bool {
            for ch in s.chars() {
                if let Some(v) = map.get_mut(&ch) {
                    *v -= 1;
                    if *v == 0 {
                        map.remove(&ch);
                    }
                } else {
                    return false;
                }
            }
            map.is_empty()
        }

        let mut res = vec![];
        if s.len() < p.len() {
            return res;
        }
        let s_str = s.as_str();

        let mut map = std::collections::HashMap::<char, i32>::new();
        for ch in p.chars() {
            let num = map.entry(ch).or_default();
            *num += 1;
        }
        for i in 0..=s.len() - p.len() {
           if is_match(&s_str[i..i+p.len()], map.clone()) {
               res.push(i as i32);
           }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![0, 6], Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()));
    assert_eq!(vec![0,1,2], Solution::find_anagrams("abab".to_string(), "ab".to_string()));
    assert_eq!(vec![0], Solution::find_anagrams("abc".to_string(), "acb".to_string()));
}