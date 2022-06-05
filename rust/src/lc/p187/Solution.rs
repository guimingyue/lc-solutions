struct Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let mut map = std::collections::HashMap::new();
        map.insert('A', 0);
        map.insert('C', 1);
        map.insert('G', 2);
        map.insert('T', 3);

        let l = 10;
        if s.len() <= l {
            return vec![];
        }
        let mask = (1 << 2 * l) - 1;

        let mut val = 0;
        let s_vec: Vec<char> = s.chars().collect();
        for i in 0..l {
            let v = map.get(&s_vec[i]).unwrap();
            val = (val << 2) | *v;
        }

        let mut cnt = std::collections::HashMap::new();
        cnt.insert(val, 1);

        let mut vec = vec![];
        for i in l..s.len() {
            let v = map.get(&s_vec[i]).unwrap();
            val = ((val << 2) | *v) & mask;
            match cnt.get(&val) {
                Some(& c) => {
                    if c == 1 {
                        vec.push(String::from(s.get(i-l+1..i+1).unwrap()));
                    }
                    cnt.insert(val, c+1);

                }
                None => {
                    cnt.insert(val, 1);
                }
            }
        }

        vec
    }
}

#[test]
fn test() {
    assert_eq!(Vec::<String>::new(), Solution::find_repeated_dna_sequences("A".to_string()));
    assert_eq!(vec!["AAAAACCCCC","CCCCCAAAAA"], Solution::find_repeated_dna_sequences("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT".to_string()));
    assert_eq!(vec!["AAAAAAAAAA"], Solution::find_repeated_dna_sequences("AAAAAAAAAAAAA".to_string()));
}