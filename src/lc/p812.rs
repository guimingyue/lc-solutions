use crate::lc::Solution;

/// #f64
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        fn area(x: &Vec<i32>, y: &Vec<i32>, z: &Vec<i32>) -> f64 {
            (x[0] * y[1] + y[0] * z[1] + z[0] * x[1] - x[0] * z[1] - y[0] * x[1] - z[0] * y[1]).abs() as f64 / 2f64
        }
        let mut res: f64 = 0.0;
        for x in 0..points.len() {
            for y in x+1..points.len() {
                for z in y+1..points.len() {
                    res = res.max(area(&points[x], &points[y], &points[z]));
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(2f64, Solution::largest_triangle_area(vec![vec![0,0],vec![0,1],vec![1,0],vec![0,2],vec![2,0]]));
    assert_eq!(0.5f64, Solution::largest_triangle_area(vec![vec![0,0],vec![0,1],vec![1,0]]));
}