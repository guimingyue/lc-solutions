use crate::lc::Solution;

/// #String
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let base = (2 * num_rows - 2) as usize;
        let mut res = String::new();
        let bytes = s.into_bytes();
        for i in 0..num_rows {
            let mut j = i as usize;
            let mut idx = (2 * num_rows - 2 - i) as usize;
            while j < bytes.len() {
                res.push(bytes[j] as char);
                if i > 0 && i < num_rows - 1 && idx < bytes.len() {
                    res.push(bytes[idx] as char);
                }
                j += base;
                idx += base;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!("PAHNAPLSIIGYIR", Solution::convert("PAYPALISHIRING".to_string(), 3));
    assert_eq!("PINALSIGYAHRPI", Solution::convert("PAYPALISHIRING".to_string(), 4));
    assert_eq!("A", Solution::convert("A".to_string(), 1));
    assert_eq!("AB", Solution::convert("AB".to_string(), 2));
    assert_eq!("ACB", Solution::convert("ABC".to_string(), 2));
}