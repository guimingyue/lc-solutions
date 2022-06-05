struct Solution {}

/// String, char
impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut pre = "1";
        let mut res = String::new();
        res.push('1');
        let n = n as usize;
        for i in 2..=n {
            let s = res.to_string();
            pre = s.as_str();
            res = String::new();
            let bytes = pre.as_bytes();
            let mut charCnt:u32 = 1;
            for k in 1..bytes.len() {
                if bytes[k] == bytes[k-1] {
                    charCnt += 1;
                } else {
                    res.push(char::from_digit(charCnt, 10).unwrap());
                    res.push(char::from(bytes[k-1]));
                    charCnt = 1;
                }
            }
            res.push(char::from_digit(charCnt, 10).unwrap());
            res.push(char::from(bytes[bytes.len()-1]))
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!("1", Solution::count_and_say(1));
    assert_eq!("11", Solution::count_and_say(2));
    assert_eq!("21", Solution::count_and_say(3));
    assert_eq!("1211", Solution::count_and_say(4));
    assert_eq!("111221", Solution::count_and_say(5));
    assert_eq!("312211", Solution::count_and_say(6));
    assert_eq!("13112221", Solution::count_and_say(7));
    assert_eq!("1113213211", Solution::count_and_say(8));
    assert_eq!("31131211131221", Solution::count_and_say(9));
    assert_eq!("13211311123113112211", Solution::count_and_say(10));
}