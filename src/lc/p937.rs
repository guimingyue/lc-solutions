use crate::lc::Solution;

/// #Vec, sort_by
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        let mut log_str = vec![];
        let mut log_dig = vec![];
        for log in &logs {
            let vecs: Vec<_> = log.as_str().split(' ').collect();
            let all_alph = vecs[1].chars().all(|ch|ch.is_ascii_alphabetic());
            if all_alph {
                log_str.push(log.as_str());
            } else {
                log_dig.push(log.as_str());
            }
        }
        log_str.sort_by(|a, b| {
            let a_sp: Vec<&str> = a.splitn(2, ' ').collect();
            let b_sp: Vec<&str> = b.splitn(2, ' ').collect();

            if a_sp[1].eq(b_sp[1]) {
                a_sp[0].cmp(b_sp[0])
            } else {
                a_sp[1].cmp(b_sp[1])
            }
        });
        let mut res = vec![];
        for log in log_str {
            res.push(log.to_string());
        }

        for log in log_dig {
            res.push(log.to_string());
        }

        res
    }
}

#[test]
fn test() {
    fn test(logs: Vec<&str>) -> Vec<String>{
        Solution::reorder_log_files(logs.iter().map(|log|log.to_string()).collect())
    }
    assert_eq!(vec!["let1 art can","let3 art zero","let2 own kit dig","dig1 8 1 5 1","dig2 3 6"],
        test(vec!["dig1 8 1 5 1","let1 art can","dig2 3 6","let2 own kit dig","let3 art zero"]));
    assert_eq!(vec!["g1 act car","a8 act zoo","ab1 off key dog","a1 9 2 3 1","zo4 4 7"],
               test(vec!["a1 9 2 3 1","g1 act car","zo4 4 7","ab1 off key dog","a8 act zoo"]));
}