use std::str::FromStr;
use crate::lc::Solution;

/// #String.split
impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let num1_str:Vec<&str> = num1.split("+").collect();
        let num2_str:Vec<&str> = num2.split("+").collect();
        let num1_v1 = i32::from_str(num1_str[0]).unwrap();
        let num1_v2 = i32::from_str(&num1_str[1][0..num1_str[1].len()-1]).unwrap();

        let num2_v1 = i32::from_str(num2_str[0]).unwrap();
        let num2_v2 = i32::from_str(&num2_str[1][0..num2_str[1].len()-1]).unwrap();

        let mut res = String::new();
        res.push_str((num1_v1 * num2_v1 - num1_v2 * num2_v2).to_string().as_str());
        res.push_str("+");
        res.push_str((num1_v1 * num2_v2 + num1_v2 * num2_v1).to_string().as_str());
        res.push_str("i");
        res
    }
}

#[test]
fn test() {
    assert_eq!("0+2i", Solution::complex_number_multiply("1+1i".to_string(), "1+1i".to_string()));
    assert_eq!("0+-2i", Solution::complex_number_multiply("1+-1i".to_string(), "1+-1i".to_string()));
    assert_eq!("1+1i", Solution::complex_number_multiply("0+1i".to_string(), "1+-1i".to_string()));
}