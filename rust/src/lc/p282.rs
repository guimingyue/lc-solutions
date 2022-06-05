struct Solution {}

/// Vec, String
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut ans = vec![];
        let mut expr = vec![];
        Solution::backtrace(num.as_bytes(), 0, &mut expr, 0, 0, &mut ans, target);
        ans
    }

    fn backtrace(bytes: &[u8], i: usize, expr: &mut Vec<u8>, res: i64, mul: i64, ans: &mut Vec<String>, target: i32) {
        if i == bytes.len() {
            if res == target as i64 {
                let s = String::from_utf8(expr.clone()).unwrap();
                ans.push(s);
            }
            return;
        }
        let expr_idx = expr.len();
        if i > 0 {
            expr.push('0' as u8);
        }
        let mut val:i64 = 0;
        for j in i..bytes.len() {
            if j != i && bytes[i] == '0' as u8 {
                break;
            }
            val = val * 10 + (bytes[j] - '0' as u8) as i64;
            expr.push(char::from(bytes[j]) as u8);
            if i == 0 {
                Solution::backtrace(bytes, j+1, expr, val, val, ans, target);
            } else {
                expr[expr_idx] = '+' as u8;
                Solution::backtrace(bytes, j+1, expr, res+val, val, ans, target);
                expr[expr_idx] = '-' as u8;
                Solution::backtrace(bytes, j+1, expr, res-val, -val, ans, target);
                expr[expr_idx] = '*' as u8;
                Solution::backtrace(bytes, j+1, expr, res-mul + mul * val, mul * val, ans, target);
            }
        }
        expr.resize(expr_idx, 0);
    }
}

#[test]
fn test() {
    assert_eq!(vec!["1+2+3", "1*2*3"], Solution::add_operators("123".to_string(), 6));
    assert_eq!(vec!["2+3*2", "2*3+2"], Solution::add_operators("232".to_string(), 8));
    assert_eq!(vec!["1*0+5","10-5"], Solution::add_operators("105".to_string(), 5));
    assert_eq!(vec!["0+0", "0-0", "0*0"], Solution::add_operators("00".to_string(), 0));
    assert_eq!(Vec::<String>::new(), Solution::add_operators("3456237490".to_string(), 9191));
}