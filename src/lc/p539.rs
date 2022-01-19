use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        if time_points.len() > 1440 {
            return 0;
        }
        fn to_minute(time: &str) -> i32 {
            let b = '0' as i32;
            let chs = time.as_bytes();
            ((chs[0] as i32  - b) * 10 + chs[1] as i32 - b) * 60 + (chs[3] as i32 - b) * 10 + chs[4] as i32 - b
        }
        let mut time_points = time_points;
        time_points.sort();
        let first = to_minute(time_points[0].as_str());
        let mut pre = first;
        let mut res = i32::MAX;
        for i in 1..time_points.len() {
            let cur = to_minute(time_points[i].as_str());
            res = res.min(cur - pre);
            if res == 0 {
                return res;
            }
            pre = cur;
        }
        res.min(first + 1440 - pre) as i32
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::find_min_difference(vec!["23:59".to_string(),"00:00".to_string()]));
    assert_eq!(0, Solution::find_min_difference(vec!["00:00".to_string(), "23:59".to_string(),"00:00".to_string()]));
    assert_eq!(60, Solution::find_min_difference(vec!["01:00".to_string(), "22:59".to_string(),"00:00".to_string()]));
    assert_eq!(30, Solution::find_min_difference(vec!["00:30".to_string(), "22:59".to_string(),"00:00".to_string()]));
}