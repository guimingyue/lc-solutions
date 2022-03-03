use core::str::FromStr;
use crate::lc::Solution;

/// #FromStr, Vec.extend, Vec.extend_from_slice
impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let len = n.len();
        let mut candidates:Vec<i64> = vec![(10 as i64).pow(len as u32 - 1) - 1, (10 as i64).pow(len as u32) + 1];
        let num = i64::from_str(n.as_str()).unwrap();
        let prefix_num = i64::from_str(&n[..(len+1)/2]).unwrap();
        for i in prefix_num-1..=prefix_num+1 {
            let mut vec = vec![];
            let mut prefix = i.to_string().into_bytes();
            vec.extend(prefix.iter());
            prefix.reverse();
            vec.extend_from_slice(&prefix[(len&1)..prefix.len()]);
            candidates.push(i64::from_str(String::from_utf8(vec).unwrap().as_str()).unwrap());
        }
        let mut res = -1;
        for v in candidates {
            if v == num {
                continue;
            }
            if res == -1 || (v - num).abs() < (num - res).abs() ||
                (v - num).abs() == (num - res).abs() && v < res {
                res = v;
            }
        }
        res.to_string()
    }
}

#[test]
fn test() {
    assert_eq!("101", Solution::nearest_palindromic("99".to_string()));
    assert_eq!("99", Solution::nearest_palindromic("101".to_string()));
    assert_eq!("1001", Solution::nearest_palindromic("1024".to_string()));
    assert_eq!("121", Solution::nearest_palindromic("123".to_string()));
    assert_eq!("0", Solution::nearest_palindromic("1".to_string()));
}