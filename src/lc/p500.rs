use crate::lc::Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut vec = vec![];
        for word in words {
            if Solution::in_one_line(word.as_str()) {
                vec.push(word);
            }
        }
        vec
    }

    fn in_one_line(word: &str) -> bool {
        const  ch_line: [i32; 26] = [2,3,3,2,1,2,2, 2,1,2,2,2,3,3, 1,1,1,1,2,1, 1,3,1,3,1,3];
        let mut chars = word.chars();
        let mut pre = 0;
        while let Some(ch) = chars.next() {
            let ch_val = ch as i32;
            let mut idx = 0;
            if ch_val >= 'a' as i32 && ch_val <= 'z' as i32 {
                idx = ch_val - 'a' as i32;
            } else {
                idx = ch_val - 'A' as i32;
            }
            if pre == 0 {
                pre = ch_line[idx as usize];
            } else {
                if pre != ch_line[idx as usize] {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(vec!["Alaska","Dad"], Solution::find_words(vec!["Hello".to_string(),"Alaska".to_string(),"Dad".to_string(),"Peace".to_string()]));
    assert_eq!(Vec::<String>::new(), Solution::find_words(vec!["omk".to_string()]));
    assert_eq!(vec!["adsdf","sfd"], Solution::find_words(vec!["adsdf".to_string(),"sfd".to_string()]));
}