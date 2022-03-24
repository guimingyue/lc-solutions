use crate::lc::Solution;

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut freq = [0; 2];
        let mut c = ' ';
        let mut cnt = 0;
        for ch in colors.chars() {
            if c != ch {
                c = ch;
                cnt = 1;
            } else {
                cnt += 1;
                if cnt > 2 {
                    freq[(c as u8 - 'A' as u8) as usize] += 1;
                }
            }
        }
        freq[0] > freq[1]
    }
}

#[test]
fn test() {
    assert!(Solution::winner_of_game("AAABABB".to_string()));
    assert!(!Solution::winner_of_game("AA".to_string()));
    assert!(!Solution::winner_of_game("ABBBBBBBAAA".to_string()));
    assert!(Solution::winner_of_game("AAAABBA".to_string()));
    assert!(Solution::winner_of_game("AAAABBB".to_string()));
}