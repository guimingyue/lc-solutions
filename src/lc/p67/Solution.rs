pub struct Solution {
    
}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut ait = a.chars().rev();
        let mut bit = b.chars().rev();
        let mut res = String::new();
        let mut carry = 0;
        let mut a_op = ait.next();
        let mut b_op = bit.next();
        loop {
            if a_op == None && b_op == None {
                break
            }
            let mut bit_a = 0;
            if a_op != None {
                bit_a = a_op.unwrap().to_digit(2).unwrap();
                a_op = ait.next();
            }
            let mut bit_b = 0;
            if b_op != None {
                bit_b = b_op.unwrap().to_digit(2).unwrap();
                b_op = bit.next();
            }
            let sum = bit_a + bit_b + carry;
            let bit = sum % 2;
            carry = sum / 2;
            res.push(char::from_digit(bit, 2).unwrap());
        }
        if carry > 0 {
            res.push(char::from_digit(carry, 10).unwrap());
        }
        res.chars().rev().collect::<String>()
    }
}