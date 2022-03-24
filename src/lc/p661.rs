use crate::lc::Solution;

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const DIRT:[(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
        let m = img.len() as i32;
        let n = img[0].len() as i32;
        let mut res = vec![vec![0; img[0].len()]; img.len()];
        for i in 0..img.len() {
            for j in 0..img[i].len() {
                let mut cell = img[i][j];
                let mut num = 1;
                for d in DIRT {
                    if (d.0 + i as i32) < 0 || (d.0 + i as i32  >= m) || (d.1 + j as i32) < 0 || (d.1 + j as i32) >= n {
                        continue;
                    }
                    cell += img[(d.0 + i as i32) as usize][(d.1 + j as i32) as usize];
                    num += 1;
                }
                res[i][j] = cell / num;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![vec![4,4,5],vec![5,6,6],vec![8,9,9],vec![11,12,12],vec![13,13,14]], Solution::image_smoother(vec![vec![2,3,4],vec![5,6,7],vec![8,9,10],vec![11,12,13],vec![14,15,16]]));
    assert_eq!(vec![vec![0,0,0], vec![0,0,0], vec![0,0,0]], Solution::image_smoother(vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]]));
    assert_eq!(vec![vec![137,141,137],vec![141,138,141],vec![137,141,137]], Solution::image_smoother(vec![vec![100,200,100],vec![200,50,200],vec![100,200,100]]));
}