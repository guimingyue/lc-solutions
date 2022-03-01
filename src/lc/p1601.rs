use crate::lc::Solution;

/// #count_ones, iter.find
impl Solution {
    pub fn maximum_requests(n: i32, requests: Vec<Vec<i32>>) -> i32 {
        let m = requests.len();
        let mut res = 0;
        for i in 0..((1 as i32) << m as i32)  {
            let cnt = i.count_ones() as i32;
            if res >=  cnt {
                continue;
            }
            let mut delta = vec![0; n as usize];
            for k in 0..m {
                if (i >> k) & 1 != 0 {
                    delta[requests[k][0] as usize] -= 1;
                    delta[requests[k][1] as usize] += 1;
                }
            }

            res = match delta.iter().find(|x| **x != 0) {
                Some(v) => res,
                None => cnt
            }

        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(5, Solution::maximum_requests(5, vec![vec![0,1],vec![1,0],vec![0,1],vec![1,2],vec![2,0],vec![3,4]]));
    assert_eq!(3, Solution::maximum_requests(3, vec![vec![0,0],vec![1,2],vec![2,1]]));
    assert_eq!(4, Solution::maximum_requests(4, vec![vec![0,3],vec![3,1],vec![1,2],vec![2,0]]));
    assert_eq!(2, Solution::maximum_requests(3, vec![vec![0,1],vec![1,2],vec![2,1]]));
    assert_eq!(0, Solution::maximum_requests(3, vec![vec![0,1],vec![1,2]]));
}