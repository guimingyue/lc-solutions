use crate::lc::Solution;

/// #Vec, String
impl Solution {

    pub fn max_product(words: Vec<String>) -> i32 {
        let mut max_product = 0;
        let mut map = vec![0;26];
        let base = 'a' as usize;
        for i in 0..words.len() {
            map.fill(0);
            for ch in words[i].chars() {
                map[ch as usize - base] = 1;
            }
            for j in i+1..words.len() {
                let mut product = true;
                for ch_j in words[j].chars() {
                    if map[ch_j as usize - base] == 1 {
                        product = false;
                        break;
                    }
                }
                if product {
                    max_product = max_product.max(words[i].len() * words[j].len());
                }

            }
        }
        max_product as i32
    }
}

#[test]
fn test() {
    assert_eq!(16, Solution::max_product(vec!["abcw".to_string(),"baz".to_string(),"foo".to_string(),"bar".to_string(),"xtfn".to_string(),"abcdef".to_string()]));
    assert_eq!(4, Solution::max_product(vec!["a".to_string(),"ab".to_string(),"abc".to_string(),"d".to_string(),"cd".to_string(),"bcd".to_string(),"abcd".to_string()]));
    assert_eq!(0, Solution::max_product(vec!["a".to_string(),"aa".to_string(),"aaa".to_string(),"aaaa".to_string()]));
    assert_eq!(0, Solution::max_product(vec!["a".to_string(),"aa".to_string()]));
    assert_eq!(1, Solution::max_product(vec!["a".to_string(),"b".to_string()]));
}