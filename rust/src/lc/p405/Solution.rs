pub struct Solution {
    
}

impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return String::from('0');
        }
        let mut res = String::new();
        for i in (0..8).rev() {
            let bits = (num >> (4 * i)) & 0xf;
            if res.len() > 0 || bits > 0{
                res.push(char::from_digit(bits as u32, 16).unwrap());
            }
        }
        res
    }
}
