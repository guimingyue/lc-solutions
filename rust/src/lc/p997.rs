use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![0; n as usize];
        let mut trust_count = vec![0; n as usize];
        let mut judge = 1;
        let mut max = 0;
        for v in trust {
            trust_count[(v[0]-1) as usize] += 1;
            let num = (v[1] - 1) as usize;
            count[num] += 1;
            if count[num] > max {
                max = count[num];
                judge = v[1];
            }
        }
        if trust_count[(judge - 1) as usize] == 0 && count[(judge - 1) as usize] == n-1 {
            judge
        } else {
            -1
        }
    }
}

#[test]
fn test() {
    assert_eq!(1, Solution::find_judge(1, vec![]));
    assert_eq!(2, Solution::find_judge(2, vec![vec![1,2]]));
    assert_eq!(3, Solution::find_judge(3, vec![vec![1,3],vec![2,3]]));
    assert_eq!(-1, Solution::find_judge(3, vec![vec![1,3],vec![2,3],vec![3,1]]));
    assert_eq!(-1, Solution::find_judge(3, vec![vec![1,2],vec![2,3]]));
    assert_eq!(3, Solution::find_judge(4, vec![vec![1,3],vec![1,4],vec![2,3],vec![2,4],vec![4,3]]));
}