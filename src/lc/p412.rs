struct Solution {}

/// String, formate!
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = vec![];
        for i in 1..=n {
            let mut s = String::new();
            if i % 3 == 0 {
                s.push_str("Fizz");
            }
            if i % 5 == 0 {
                s.push_str("Buzz");
            }
            if s.is_empty() {
                s.push_str(format!("{}", i).as_str());
            }
            res.push(s);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec!["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"],
               Solution::fizz_buzz(15));
    assert_eq!(vec!["1","2"], Solution::fizz_buzz(2));
    assert_eq!(vec!["1","2","Fizz"], Solution::fizz_buzz(3));
    assert_eq!(vec!["1","2","Fizz","4","Buzz","Fizz"], Solution::fizz_buzz(6));
}