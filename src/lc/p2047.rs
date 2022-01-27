use crate::lc::Solution;

/// [], tuple
impl Solution {
    pub fn count_valid_words(sentence: String) -> i32 {

        fn to_letter_or_end(vec: &[u8], idx: &mut usize) {
            while *idx < vec.len() && vec[*idx] == ' ' as u8 {
                *idx += 1;
            }
        }

        fn to_space_or_end(vec: &[u8], idx: &mut usize) {
            while *idx < vec.len() && vec[*idx] != ' ' as u8 {
                *idx += 1;
            }
        }

        fn next_token(vec: &[u8], idx: &mut usize) -> (usize, usize) {
            let start = *idx;
            to_space_or_end(vec, idx);
            (start, *idx - 1)
        }

        fn is_valid_token(vec: &[u8], start: usize, end: usize) -> bool {
            if start > end {
                return false;
            }
            if start == end && (vec[start] == '!' as u8 || vec[start] == '.' as u8 || vec[start] == ',' as u8) {
                return true;
            }
            if vec[start] < 'a' as u8 || vec[start] > 'z' as u8 {
                return false;
            }
            let mut concat_num = 0;
            for i in start..=end {
                if vec[i] == '-' as u8 {
                    concat_num += 1;
                    if concat_num > 1 {
                        return false;
                    }
                    if i == end || !char::from(vec[i-1]).is_alphabetic() || !char::from(vec[i+1]).is_alphabetic() {
                        return false;
                    }
                } if vec[i] == '!' as u8 || vec[i] == '.' as u8 || vec[i] == ',' as u8 {
                    if i == end {
                        return true;
                    } else {
                        return false;
                    }
                } if vec[i] >= '0' as u8 && vec[i] <= '9' as u8 {
                    return false;
                }
            }
            true
        }

        let vec = sentence.as_bytes();
        let mut idx = 0;
        let mut res = 0;
        while idx < vec.len() {
            let (start, end) = next_token(vec, &mut idx);
            if is_valid_token(vec, start, end) {
                res += 1;
            }
            to_letter_or_end(vec, &mut idx);
        }
        res
    }
}


#[test]
fn test() {
    assert_eq!(4, Solution::count_valid_words(". ! 7hk  al6 l! aon49esj35la k3 7u2tkh  7i9y5  !jyylhppd et v- h!ogsouv 5".to_string()));
    assert_eq!(1, Solution::count_valid_words("!".to_string()));
    assert_eq!(4, Solution::count_valid_words("cat ! and  dog".to_string()));
    assert_eq!(3, Solution::count_valid_words("cat and  dog".to_string()));
    assert_eq!(0, Solution::count_valid_words("!this  1-s b8d!".to_string()));
    assert_eq!(5, Solution::count_valid_words("alice and  bob are playing stone-game10".to_string()));
    assert_eq!(6, Solution::count_valid_words("he bought 2 pencils, 3 erasers, and 1  pencil-sharpener.".to_string()));
}