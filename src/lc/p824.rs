use std::io::Write;
use crate::lc::Solution;

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        fn read_word<'a>(bytes: &'a [u8], i: &mut usize) -> &'a [u8] {
            let start = *i;
            while *i < bytes.len() && bytes[*i] != ' ' as u8 {
                *i += 1;
            }
            &bytes[start..*i]
        }
        let mut vec = vec![];
        let mut a_vec = vec!['a' as u8];
        let mut i = 0;
        let bytes = sentence.as_bytes();
        while i < bytes.len() {
            let word = read_word(bytes, &mut i);
            let mut ch = (word[0] as char).to_ascii_lowercase();
            if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
                vec.write_all(word);
            } else {
                vec.write_all(&word[1..word.len()]);
                vec.push(word[0]);
            }
            vec.write_all("ma".as_bytes());
            vec.write_all(a_vec.as_slice());
            if i < bytes.len() {
                vec.push(bytes[i]);
            }
            i += 1;
            a_vec.push('a' as u8);
        }
        unsafe {String::from_utf8_unchecked(vec)}
    }
}

#[test]
fn test() {
    assert_eq!("applemaa", Solution::to_goat_latin("apple".to_string()));
    assert_eq!("Imaa peaksmaaa oatGmaaaa atinLmaaaaa", Solution::to_goat_latin("I speak Goat Latin".to_string()));
    assert_eq!("heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa",
               Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()));
}