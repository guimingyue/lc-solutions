pub mod lc;

fn main() {
    p13();
    p9();
}


fn p13() {
    assert_eq!(4, lc::p13::Solution::Solution::roman_to_int(String::from("IV")));
    assert_eq!(3, lc::p13::Solution::Solution::roman_to_int(String::from("III")));
    assert_eq!(9, lc::p13::Solution::Solution::roman_to_int(String::from("IX")));
    assert_eq!(58, lc::p13::Solution::Solution::roman_to_int(String::from("LVIII")));
    assert_eq!(1994, lc::p13::Solution::Solution::roman_to_int(String::from("MCMXCIV")));
    assert_eq!(49, lc::p13::Solution::Solution::roman_to_int(String::from("XLIX")));
    assert_eq!(999, lc::p13::Solution::Solution::roman_to_int(String::from("CMXCIX")));
}

fn p9() {
    assert!(lc::p9::Solution::Solution::is_palindrome(3));
    assert!(!lc::p9::Solution::Solution::is_palindrome(13));
    assert!(lc::p9::Solution::Solution::is_palindrome(131));
    assert!(!lc::p9::Solution::Solution::is_palindrome(-131));
}