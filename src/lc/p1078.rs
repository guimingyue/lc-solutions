use crate::lc::Solution;

/// #Vec, String, match-if
impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        fn equals(text: &str, word: &str) -> bool {
            text.eq(word)
        }
        fn next_word(text: &str, idx: usize) -> String {
            let mut res = String::new();
            for i in idx..text.len() {
                match text.get(i..i+1) {
                    Some(v) if !v.eq(" ") => res.push_str(v),
                    _ => break,
                }
            }
            res
        }
        fn next_space(text: &str) -> usize {
            let mut i = 0;
            while i < text.len() {
                if let Some(v) = text.get(i..i+1) {
                    if v.eq(" ") {
                        break;
                    }
                }
                i += 1;
            }
            i + 1
        }
        let mut res = vec![];
        let l1 = first.len();
        let l2 = second.len();
        let end_idx = text.len() - l1 -l2 - 1;
        let mut i = 0;
        while i < end_idx {
            if equals(&text[i..i+l1], first.as_str()) && equals(&text[i+l1+1..i+l1+1+l2], second.as_str()) {
                let end = i+l1+1+l2+1;
                res.push(next_word(text.as_str(), end));
                i = i+l1+1
            } else {
                i += next_space(&text[i+1..text.len()]) + 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec!["student"], Solution::find_ocurrences("alice is aa good girl she is a good student".to_string(), "a".to_string(), "good".to_string()));
    assert_eq!(vec!["girl","student"], Solution::find_ocurrences("alice is a good girl she is a good student".to_string(), "a".to_string(), "good".to_string()));
    assert_eq!(vec!["we","we","will"], Solution::find_ocurrences("we we we we will rock you".to_string(), "we".to_string(), "we".to_string()));
    assert_eq!(vec!["we","rock"], Solution::find_ocurrences("we will we will rock you".to_string(), "we".to_string(), "will".to_string()));
    assert_eq!(vec!["we","rock"], Solution::find_ocurrences("we will we will rock".to_string(), "we".to_string(), "will".to_string()));
}