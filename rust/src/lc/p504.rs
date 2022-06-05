use crate::lc::Solution;

/// #Vec.reverse
impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        if num == 0{
            return "0".to_string();
        }
        let mut vec = vec![];
        let mut n = num.abs();
        while n > 0 {
            vec.push(char::from_digit((n % 7) as u32, 10).unwrap() as u8);
            n = n / 7;
        }

        if num < 0 {
            vec.push('-' as u8)
        }
        vec.reverse();
        String::from_utf8(vec).unwrap()
    }
}

#[test]
fn test() {
    assert_eq!("202", Solution::convert_to_base7(100));
    assert_eq!("-10", Solution::convert_to_base7(-7));
    assert_eq!("130", Solution::convert_to_base7(70));
    assert_eq!("-130", Solution::convert_to_base7(-70));
}