pub struct Solution {
    
}

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut words = std::collections::HashMap::new();
        for item in paths.iter() {
            let src = item.get(0).unwrap();
            let dest = item.get(1).unwrap();

            if words.contains_key(src) {
                let count = words.get(src).unwrap();
                words.insert(src, count+1);
            } else {
                words.insert(src, 1);
            }

            if !words.contains_key(dest) {
                words.insert(dest, 0);
            }
        }
        let mut res = String::from("");
        for (k, &v) in &words {
            if v == 0 {
                res.push_str(k);
                break;
            }
        }
        res
    }
}