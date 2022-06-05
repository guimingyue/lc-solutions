
struct Solution {}

/// #Vec
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        res.push(vec![1]);
        let n = num_rows as usize;
        for i in 2..=n {
            let mut vec = res.last().unwrap();
            let mut new_vec = vec![];
            new_vec.push(1);
            for j in 0..(vec.len() - 1)  {
                new_vec.push(vec[j] + vec[j+1]);
            }
            new_vec.push(1);
            res.push(new_vec);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]], Solution::generate(5));
    assert_eq!(vec![vec![1]], Solution::generate(1));
}