pub struct Solution {
    
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut num = 0;
        let mut x_num = x;
        while x_num > 0 {
            num = num * 10 + x_num % 10;
            x_num = x_num / 10;
        }
        num == x
    }
}