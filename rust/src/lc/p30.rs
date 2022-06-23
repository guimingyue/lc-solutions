use std::collections::HashMap;
use crate::lc::Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut res = vec![];
        let m = words.len();
        let n = words[0].len();
        for i in 0..n {
            if i + m * n > s.len() {
                break;
            }
            let mut map:HashMap<&str, i32> = HashMap::new();
            for j in 0..words.len() {
                let cnt = map.entry(&s[i + j * n..i + (j+1) * n]).or_default();
                *cnt += 1;
            }
            for word in &words {
                let cnt = map.entry(word.as_str()).or_default();
                *cnt -= 1;
                if *cnt == 0 {
                    map.remove(word.as_str());
                }
            }
            let mut start = i;
            while start < s.len() - m * n + 1 {
                if start != i {
                    let word = &s[start + (m - 1) * n..start + m * n];
                    let inc_cnt = map.entry(word).or_default();
                    *inc_cnt += 1;
                    if *inc_cnt == 0 {
                        map.remove(word);
                    }
                    let word = &s[start-n..start];
                    let des_cnt = map.entry(word).or_default();
                    *des_cnt -= 1;
                    if *des_cnt == 0 {
                        map.remove(word);
                    }
                }
                if map.is_empty() {
                    res.push(start as i32);
                }
                start += n;
            }
        }
        res
    }
}

#[test]
fn test() {
    fn test(s: &str, words: Vec<&str>) -> Vec<i32>{
        let ws = words.iter().map(|w|(*w).to_string()).collect();
        Solution::find_substring(s.to_string(), ws)
    }
    assert_eq!(vec![0, 9], test("barfoothefoobarman", vec!["foo", "bar"]));
    assert_eq!(Vec::<i32>::new(), test("wordgoodgoodgoodbestword", vec!["word","good","best","word"]));
    assert_eq!(vec![6, 9,12], test("barfoofoobarthefoobarman", vec!["bar","foo","the"]));
}