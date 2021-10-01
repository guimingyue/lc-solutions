pub mod lc;

fn main() {
    p13();
    p9();
    p6();
    p1436();
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

fn p6() {
    assert_eq!(vec![1, 2, 4], lc::p6::Solution::Solution::plus_one(vec![1, 2, 3]));
    assert_eq!(vec![4, 3, 2, 2], lc::p6::Solution::Solution::plus_one(vec![4, 3, 2, 1]));
    assert_eq!(vec![1], lc::p6::Solution::Solution::plus_one(vec![0]));
    assert_eq!(vec![1,0], lc::p6::Solution::Solution::plus_one(vec![9]));
}

fn p1436() {
    assert_eq!("Sao Paulo", lc::p1436::Solution::Solution::dest_city(vec![
        vec![String::from("London"), String::from("New York")],
        vec![String::from("New York"), String::from("Lima")],
        vec![String::from("Lima"), String::from("Sao Paulo")],
    ]));

    assert_eq!("D", lc::p1436::Solution::Solution::dest_city(vec![
        vec![String::from("A"), String::from("B")],
        vec![String::from("B"), String::from("C")],
        vec![String::from("C"), String::from("D")],
    ]));
}