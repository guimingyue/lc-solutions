use crate::lc::Solution;

/// #Vec, match
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        fn idx(ch: char) -> usize {
            let idx = match ch {
                'A'..='Z' =>
                    ch as u8 - 'A' as u8,
                'a'..='z' =>
                    ch as u8 - 'a' as u8,
                _=> 26
            };
            idx as usize
        }

        fn map(str: &str) -> [i32;27] {
            let mut map = [0;27];
            for ch in str.chars() {
                let idx = idx(ch);
                map[idx] += 1;
            }
            map
        }

        fn distance(str: &str, map: [i32; 27]) -> i32 {
            let mut word_map = map.clone();
            for ch in str.chars() {
                let idx = idx(ch);
                if word_map[idx] > 0 {
                    word_map[idx] -= 1;
                }
            }
            let mut distice = 0;
            for i in 0..27 {
                if map[i] >= word_map[i] {
                    distice += map[i] - word_map[i];
                }
            }
            distice
        }

        let arr = map(license_plate.as_str());
        let mut max_distince = 0;
        let mut res = String::new();
        for word in words {
            let dis = distance(word.as_str(), arr);
            if dis > max_distince || (dis == max_distince && word.len() < res.len()) {
                max_distince = dis;
                res.clear();
                res.push_str(word.as_str());
            }
        }
        res
    }
}

#[test]
fn test() {

    assert_eq!("steps".to_string(), Solution::shortest_completing_word("1s3 PSt".to_string(),
                                                                       vec!["step".to_string(), "steps".to_string(), "stripe".to_string(), "stepple".to_string()]));
    assert_eq!("pest".to_string(), Solution::shortest_completing_word("1s3 456".to_string(),
                                                                      vec!["looks".to_string(), "pest".to_string(), "stew".to_string(), "show".to_string()]));
    assert_eq!("husband".to_string(), Solution::shortest_completing_word("Ah71752".to_string(),
                                                                         str_vec_to_string_vec(vec!["suggest","letter","of","husband","easy","education","drug","prevent","writer","old"])));
    assert_eq!("enough".to_string(), Solution::shortest_completing_word("OgEu755".to_string(),
                                                                        str_vec_to_string_vec(vec!["enough","these","play","wide","wonder","box","arrive","money","tax","thus"])));
    assert_eq!("simple".to_string(), Solution::shortest_completing_word("iMSlpe4".to_string(),
                                                                        vec!["claim".to_string(),"consumer".to_string(),"student".to_string(),"camera".to_string(),"public".to_string(),"never".to_string(),"wonder".to_string(),"simple".to_string(),"thought".to_string(),"use".to_string()]));

}

fn str_vec_to_string_vec(strs: Vec<&str>) -> Vec<String> {
    let mut vec = vec![];
    for str in strs {
        vec.push(String::from(str));
    }
    vec
}