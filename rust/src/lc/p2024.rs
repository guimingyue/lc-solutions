use crate::lc::Solution;

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        fn max_consecutive_char(bytes: &[u8], ch: u8, k: i32) -> i32 {
            fn equals(l: u8, r: u8) -> i32 {
                if l == r {
                    0
                } else {
                    1
                }
            }
            let mut ans = 0;
            let (mut left, mut right) = (0, 0);
            let mut sum = 0;
            while right < bytes.len() {
                sum += equals(bytes[right], ch);
                while sum > k {
                    sum -= equals(bytes[left], ch);
                    left += 1;
                }
                ans = ans.max(right - left + 1);
                right += 1;
            }
            ans as i32
        }
        let bytes = answer_key.as_bytes();
        max_consecutive_char(bytes, 'T' as u8, k).max(max_consecutive_char(bytes, 'F' as u8, k))
    }
}

#[test]
fn test() {
    assert_eq!(4, Solution::max_consecutive_answers("TTFF".to_string(), 2));
    assert_eq!(3, Solution::max_consecutive_answers("TFFT".to_string(), 1));
    assert_eq!(5, Solution::max_consecutive_answers("TTFTTFTT".to_string(), 1));
    assert_eq!(4, Solution::max_consecutive_answers("TTFFTFTT".to_string(), 1));
    assert_eq!(3, Solution::max_consecutive_answers("TFTFTF".to_string(), 1));
    assert_eq!(2, Solution::max_consecutive_answers("TF".to_string(), 1));
    assert_eq!(3, Solution::max_consecutive_answers("TFFT".to_string(), 1));
}