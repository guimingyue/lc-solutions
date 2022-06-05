use crate::lc::Solution;
use std::collections::HashSet;

/// #HashSet
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        const MOS: [&str; 26] = [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."];

        let mut set = HashSet::new();
        for word in &words {
            let mut word_mos = String::new();
            for ch in word.chars() {
                word_mos.push_str(MOS[(ch as u8 - 'a' as u8) as usize]);
            }
            set.insert(word_mos);
        }
        println!("{:?}", set);
        set.len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::unique_morse_representations(vec!["gin".to_string(), "zen".to_string(), "gig".to_string(), "msg".to_string()]));
    assert_eq!(1, Solution::unique_morse_representations(vec!["a".to_string()]));
}