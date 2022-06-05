pub struct Solution {
    
}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut trimed = String::new();
        for ch in s.chars() {
            if ch.is_digit(10) || ch.is_alphabetic() {
                trimed.push(ch);
            }
        }
        let lower_s = trimed.to_lowercase();
        let new_s = String::from(&lower_s);
        let mut s_r = new_s.chars().rev();
        let mut s_s = lower_s.chars();
        let mut s_s_op = s_s.next();
        let mut s_r_op = s_r.next();
        loop {
            if s_s_op == None || s_r_op == None {
                break
            }
            if s_s_op.unwrap() != s_r_op.unwrap() {
                return false;
            }
            s_s_op = s_s.next();
            s_r_op = s_r.next();
        }

        s_s_op == None && s_r_op == None
    }
}