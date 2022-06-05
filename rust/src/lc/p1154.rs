use crate::lc::Solution;

/// #match, match-if, const
impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        const days:[i32; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
        let mut state = 0;
        let mut v = 0;
        let mut plus_one = false;
        let mut month = 0;
        for ch in date.chars() {
            if ch == '-' {
                match state {
                    0 if (v % 4 == 0 && v % 100 != 0) || v % 400 == 0 => plus_one = true,
                    1 => month = v as usize,
                    _ => (),
                }
                v = 0;
                state += 1;
            } else {
                v = v * 10 + (ch as u8 - '0' as u8) as i32;
            }
        }

        let mut res = days[month - 1] + v;
        if month > 2 && plus_one {
            res += 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::day_of_year("2004-01-01".to_string()));
    assert_eq!(61, Solution::day_of_year("2004-03-01".to_string()));
    assert_eq!(9, Solution::day_of_year("2019-01-09".to_string()));
    assert_eq!(41, Solution::day_of_year("2019-02-10".to_string()));
    assert_eq!(60, Solution::day_of_year("2003-03-01".to_string()));
    assert_eq!(365, Solution::day_of_year("2003-12-31".to_string()));
    assert_eq!(366, Solution::day_of_year("2004-12-31".to_string()));
}