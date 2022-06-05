use crate::lc::Solution;

/// #Vec, parse
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        fn dfs(str: &str, idx: usize, sb: &mut Vec<String>, res: &mut Vec<String>) {
            if idx == str.len() {
                if sb.len() < 4 {
                    return;
                }
                let mut item = String::new();
                for (idx, v) in sb.iter().enumerate() {
                    item.push_str(v);
                    if idx < sb.len() -1 {
                       item.push_str(".");
                    }
                }
                res.push(item);
                return;
            }
            if sb.len() >= 4 {
                return;
            }
            for i in 1..=3 {
                let end = idx + i;
                if end > str.len() {
                    break;
                }
                let numstr = &str[idx..end];
                let num = numstr.parse::<i32>().unwrap();
                if num > 255 {
                    return;
                }
                sb.push(String::from(numstr));
                dfs(str, idx + i, sb, res);
                sb.pop();
                if num == 0 {
                    return;
                }
            }
        }

        let mut res = vec![];
        let mut sb = Vec::new();
        dfs(s.as_str(), 0, &mut sb, &mut res);
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec!["255.255.11.135","255.255.111.35"], Solution::restore_ip_addresses("25525511135".to_string()));
    assert_eq!(vec!["0.0.0.0"], Solution::restore_ip_addresses("0000".to_string()));
    assert_eq!(vec!["1.1.1.1"], Solution::restore_ip_addresses("1111".to_string()));
    assert_eq!(vec!["0.10.0.10","0.100.1.0"], Solution::restore_ip_addresses("010010".to_string()));
    assert_eq!(vec!["1.0.10.23","1.0.102.3","10.1.0.23","10.10.2.3","101.0.2.3"], Solution::restore_ip_addresses("101023".to_string()));
}