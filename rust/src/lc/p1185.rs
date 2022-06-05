use crate::lc::Solution;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        const month_days:[i32; 13] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];
        let mut days = (year - 1 - 1970) * 365 + (year - 1969) / 4 + month_days[month as usize - 1] + day;
        if ((year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)) && (month >= 3) {
            days += 1;
        }
        const day_of_week: [&str; 7] = ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        String::from(day_of_week[(days as usize + 4) % 7])
    }
}

#[test]
fn test() {
    assert_eq!("Sunday", Solution::day_of_the_week(28, 2, 2100));
    assert_eq!("Monday", Solution::day_of_the_week(29, 2, 2016));
    assert_eq!("Saturday", Solution::day_of_the_week(31, 8, 2019));
    assert_eq!("Sunday", Solution::day_of_the_week(18, 7, 1999));
    assert_eq!("Sunday", Solution::day_of_the_week(15, 8, 1993));
    assert_eq!("Monday", Solution::day_of_the_week(3, 1, 2022));
}