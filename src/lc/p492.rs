struct Solution {}

/// #sqrt, Vec
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let mut w = (area as f64).sqrt() as i32;
        while area % w != 0 {
            w -= 1;
        }
        vec![area/w, w]
    }
}

#[test]
fn test() {
    assert_eq!(vec![2, 2], Solution::construct_rectangle(4));
    assert_eq!(vec![3,1], Solution::construct_rectangle(3));
    assert_eq!(vec![5,1], Solution::construct_rectangle(5));
    assert_eq!(vec![5,3], Solution::construct_rectangle(15));
    assert_eq!(vec![5,4], Solution::construct_rectangle(20));
    assert_eq!(vec![5,5], Solution::construct_rectangle(25));
}