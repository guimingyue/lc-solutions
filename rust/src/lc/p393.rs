use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        fn num_of_bytes(v: i32) -> usize {
            let mut bytes_num = 0;
            let mut shift = 7;
            while (v >> shift) & 1 == 1 {
                bytes_num += 1;
                shift -= 1;
            }
            bytes_num
        }

        fn is_valid(v: i32) -> bool {
            (v >> 7) & 1 == 1 && (v >> 6) & 1 == 0
        }

        let mut i = 0;
        while i < data.len() {
            let bytes_num = num_of_bytes(data[i]);
            if bytes_num == 1 || bytes_num > 4 {
                return false;
            }
            if i + bytes_num > data.len() {
                return false;
            }
            for k in 1..bytes_num {
                if !is_valid(data[i + k]) {
                    return false;
                }
            }
            i += if bytes_num == 0 {
                1
            } else {
                bytes_num
            };
        }
        true
    }
}

#[test]
fn test() {
    assert!(Solution::valid_utf8(vec![197,130,1]));
    assert!(!Solution::valid_utf8(vec![240,162,138,147,145]));
    assert!(!Solution::valid_utf8(vec![235,140,4]));
}