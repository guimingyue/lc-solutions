pub struct Solution {
    
}

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let str = s.as_str();
        let mut res = 0;
        let mut word_entered = false;
        for ch in str.chars() {
            if word_entered {
                if ch == ' '{
                    word_entered = false;
                }
            } else if ch != ' ' {
                word_entered = true;
                res += 1;
            }
        }
        res
    }
}