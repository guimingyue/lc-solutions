use crate::lc::Solution;

/// #HashMap, Array
impl Solution {

    pub fn original_digits(s: String) -> String {
        let mut map = std::collections::HashMap::new();
        for ch in s.chars() {
            let mut v = map.entry(ch).or_default();
            *v += 1;
        }

        let mut cnt = [0; 10];
        cnt[0] = *map.get(&'z').unwrap_or(&0);
        cnt[2] = *map.get(&'w').unwrap_or(&0);
        cnt[4] = *map.get(&'u').unwrap_or(&0);
        cnt[6] = *map.get(&'x').unwrap_or(&0);
        cnt[8] = *map.get(&'g').unwrap_or(&0);
        cnt[3] = *map.get(&'h').unwrap_or(&0) - cnt[8];
        cnt[5] = *map.get(&'f').unwrap_or(&0) - cnt[4];
        cnt[7] = *map.get(&'s').unwrap_or(&0) - cnt[6];
        cnt[1] = *map.get(&'o').unwrap_or(&0) - cnt[0] - cnt[2] - cnt[4];
        cnt[9] = *map.get(&'i').unwrap_or(&0) - cnt[5] - cnt[6] - cnt[8];

        let mut res = String::new();
        for i in 0..10 {
            for j in 1..=cnt[i] {
                res.push(char::from_digit(i as u32, 10).unwrap());
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!("012".to_string(), Solution::original_digits("owoztneoer".to_string()));
    assert_eq!("45".to_string(), Solution::original_digits("fviefuro".to_string()));
    assert_eq!("3".to_string(), Solution::original_digits("ereht".to_string()));
}