pub struct Solution {
    
}

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let chs = s.chars().filter(|&x| x != '-').collect::<String>().to_uppercase();
        let ch_count = chs.len();

        let mut res = String::new();
        let rest = ch_count % (k as usize);
        let mut idx = 0;
        if rest > 0 {
            res.push_str(&chs[0..rest]);
            if rest < chs.len() {
                res.push('-');
            }
            idx += rest;
        }
        while idx < chs.len() {
            let next = idx + k as usize;
            res.push_str(&chs[idx..next]);
            if next < chs.len() {
                res.push('-');
            }
            idx = next;
        }
        res
    }
}