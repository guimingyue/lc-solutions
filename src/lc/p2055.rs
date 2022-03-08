use crate::lc::Solution;

/// #Vec.binary_search
impl Solution {
    pub fn plates_between_candles(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut candle_idx = vec![];
        for (idx, ch) in s.chars().enumerate() {
            if ch == '|' {
                candle_idx.push(idx);
            }
        }
        if candle_idx.len() == 0 {
            return vec![0; queries.len()];
        }

        let mut res = vec![];
        for query in queries {
            if query[0] == query[1] {
                res.push(0);
                continue;
            }
            let left = candle_idx.binary_search(&(query[0] as usize)).unwrap_or_else(|x|x);
            let right = candle_idx.binary_search(&(query[1] as usize)).unwrap_or_else(|x|x-1);
            let mut plate_num = candle_idx[right] as i32 - candle_idx[left] as i32 - right as i32 + left as i32;
            if plate_num < 0 {
                plate_num = 0;
            }
            res.push(plate_num);
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![0], Solution::plates_between_candles("||*".to_string(), vec![vec![2,2]]));
    assert_eq!(vec![0,0], Solution::plates_between_candles("*****".to_string(), vec![vec![2,4], vec![1,5]]));
    assert_eq!(vec![2,3], Solution::plates_between_candles("**|**|***|".to_string(), vec![vec![2,5], vec![5,9]]));
    assert_eq!(vec![9,0,0,0,0], Solution::plates_between_candles("***|**|*****|**||**|*".to_string(), vec![vec![1,17],vec![4,5],vec![14,17],vec![5,11],vec![15,16]]));
}