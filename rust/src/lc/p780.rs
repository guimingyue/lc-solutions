use crate::lc::Solution;

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let (mut tx, mut ty) = (tx, ty);
        while tx != ty && tx > sx && ty > sy {
            if tx > ty {
                tx %= ty;
            } else {
                ty %= tx;
            }
        }
        if tx == sx && ty == sy {
            true
        } else if tx == sx {
            ty > sy && (ty - sy) % sx == 0
        } else if ty == sy {
            tx > sx && (tx - sx) % sy == 0
        } else {
            false
        }
    }
}

#[test]
fn test() {
    assert!(Solution::reaching_points(9, 10, 9, 19));
    assert!(Solution::reaching_points(1, 1, 3, 5));
    assert!(!Solution::reaching_points(9, 5, 12, 8));
    assert!(!Solution::reaching_points(1, 1, 2, 2));
    assert!(Solution::reaching_points(1, 1, 1, 1));
    assert!(Solution::reaching_points(1, 1, 1, 2));
    assert!(Solution::reaching_points(1, 1, 1, 3));
    assert!(Solution::reaching_points(1, 1, 1, 4));
    assert!(Solution::reaching_points(2, 2, 2, 4));
    assert!(Solution::reaching_points(2, 2, 2, 6));
}