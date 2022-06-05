use crate::lc::Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut vec = vec![-1; s.len() + 1];
        let mut n = s.len() as i32;
        let mut idx = s.len();
        let bytes = s.into_bytes();
        while idx > 0 {
            idx -= 1;
            if bytes[idx] == 'I' as u8 {
                vec[idx + 1 ]= n;
                n -= 1;
            }
        }


        let mut idx = 0;
        while idx < bytes.len() {
            if bytes[idx] == 'D' as u8 {
                if vec[idx] == -1 {
                    vec[idx] = n;
                } else {
                    vec[idx + 1] = n;
                }
                n -= 1;
            }
            idx += 1;
        }

        for i in (0..=bytes.len()).rev() {
            if vec[i] == -1 {
                vec[i] = n;
                n -= 1;
            }
        }
        vec
    }
}

#[test]
fn test() {
    assert_eq!(vec![0,3,2,4,1], Solution::di_string_match("IDID".to_string()));
    assert_eq!(vec![0,1,2,3], Solution::di_string_match("III".to_string()));
    assert_eq!(vec![2, 1, 0, 3], Solution::di_string_match("DDI".to_string()));
}