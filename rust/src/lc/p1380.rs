use crate::lc::Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut row:Vec<usize> = vec![];
        for i in 0..matrix.len() {
            let mut min: usize = 0;
            for j in 1..matrix[i].len() {
                if matrix[i][min] > matrix[i][j] {
                    min = j;
                }
            }
            row.push(min);
        }

        let mut res = vec![];
        for i in 0..row.len() {
            let col = row[i];
            let mut r = 0;
            while r < matrix.len() {
                if i == r {
                    r += 1;
                    continue;
                }
                if matrix[r][col] > matrix[i][col] {
                    break;
                }
                r += 1;
            }
            if r == matrix.len() {
                res.push(matrix[i][col]);
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(Vec::<i32>::new(), Solution::lucky_numbers(vec![vec![1,4],vec![5,2]]));
    assert_eq!(vec![1], Solution::lucky_numbers(vec![vec![1]]));
    assert_eq!(vec![9], Solution::lucky_numbers(vec![vec![3,7,8],vec![10,11,9],vec![15,16,6]]));
    assert_eq!(vec![15], Solution::lucky_numbers(vec![vec![3,7,8],vec![9,11,13],vec![15,16,17]]));
    assert_eq!(vec![12], Solution::lucky_numbers(vec![vec![1,10,4,2],vec![9,3,8,7],vec![15,16,17,12]]));
    assert_eq!(vec![7], Solution::lucky_numbers(vec![vec![7,8],vec![1,2]]));
}