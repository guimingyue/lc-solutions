pub struct Solution {
    
}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut carry = 1;
        let mut idx = digits.len();
        while idx > 0 {
            let n = digits[idx-1] + carry;
            if n >= 10 {
                carry = 1;
            } else {
                carry = 0;
            }
            res.push(n % 10);
            idx = idx.checked_sub(1).unwrap();
        }
        if carry == 1 {
            res.push(carry);
        }
        res.reverse();
        res
    }
}