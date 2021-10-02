pub mod lc;

fn main() {
    p13();
    p9();
    p66();
    p1436();
    p405();
    p67();
    p125();
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

fn p66() {
    assert_eq!(vec![1, 2, 4], lc::p66::Solution::Solution::plus_one(vec![1, 2, 3]));
    assert_eq!(vec![4, 3, 2, 2], lc::p66::Solution::Solution::plus_one(vec![4, 3, 2, 1]));
    assert_eq!(vec![1], lc::p66::Solution::Solution::plus_one(vec![0]));
    assert_eq!(vec![1,0], lc::p66::Solution::Solution::plus_one(vec![9]));
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

fn p405() {
    assert_eq!("2a", lc::p405::Solution::Solution::to_hex(42));
    assert_eq!("1a", lc::p405::Solution::Solution::to_hex(26));
    assert_eq!("ffffffff", lc::p405::Solution::Solution::to_hex(-1));
    assert_eq!("1", lc::p405::Solution::Solution::to_hex(1));
    assert_eq!("a", lc::p405::Solution::Solution::to_hex(10));
}

fn p67() {
    assert_eq!("100", lc::p67::Solution::Solution::add_binary(String::from("11"), String::from("1")));
    assert_eq!("10101", lc::p67::Solution::Solution::add_binary(String::from("1010"), String::from("1011")));
    assert_eq!("10", lc::p67::Solution::Solution::add_binary(String::from("1"), String::from("1")));
    assert_eq!("1", lc::p67::Solution::Solution::add_binary(String::from("1"), String::from("0")));
}

fn p125() {
    assert!(lc::p125::Solution::Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()));
    assert!(!lc::p125::Solution::Solution::is_palindrome("race a car".to_string()));
    assert!(!lc::p125::Solution::Solution::is_palindrome("rac".to_string()));
    assert!(lc::p125::Solution::Solution::is_palindrome("rar".to_string()));
    assert!(!lc::p125::Solution::Solution::is_palindrome("r a".to_string()));
    assert!(lc::p125::Solution::Solution::is_palindrome("a".to_string()));
    assert!(lc::p125::Solution::Solution::is_palindrome(" a".to_string()));
}