pub struct Solution {
    
}

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let mut res = String::new();
        if (numerator < 0 && denominator > 0) || (numerator > 0 && denominator < 0) {
            res.push('-');
        }

        let mut nume = (numerator as i64).abs();
        let mut deno = (denominator as i64).abs();

        res.push_str((nume / deno).to_string().as_str());
        let mut nume = (nume % deno);
        if nume == 0 {
            return res;
        }

        let mut visited = vec![];
        let mut res_bits = vec![];

        visited.push(nume);
        res.push_str(".");
        nume = nume * 10;
        loop {
            if nume >= deno {
                let div = nume / deno;
                nume = nume % deno;
                let ch = char::from_digit(div as u32, 10).unwrap();
                res_bits.push(ch);
                if visited.contains(&nume) {
                    break;
                }
                visited.push(nume);
            } else {
                visited.push(nume);
                res_bits.push('0');
            }

            if nume == 0 {
                res.push_str(res_bits.iter().collect::<String>().as_str());
                return res;
            }
            nume = nume * 10;
        }

        let position = visited.iter().position(|item| *item == nume).unwrap();
        let (left , right) = res_bits.split_at(position);
        res.push_str(left.iter().collect::<String>().as_str());
        res.push('(');
        res.push_str(right.iter().collect::<String>().as_str());
        res.push(')');
        res
    }
}