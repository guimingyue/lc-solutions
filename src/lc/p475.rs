use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut radius = 0;
        let mut houses = houses;
        houses.sort();
        let mut heaters = heaters;
        heaters.sort();
        for h in houses {
            let mut min = (heaters[0] - h).abs();
            for i in 1..heaters.len() {
                let l = (heaters[i] - h).abs();
                if l <= min {
                    min = l;
                } else {
                    break;
                }
            }
            radius = radius.max(min);
        }
        radius
    }
}

#[test]
fn test() {
    assert_eq!(161834419, Solution::find_radius(vec![282475249,622650073,984943658,144108930,470211272,101027544,457850878,458777923], vec![823564440,115438165,784484492,74243042,114807987,137522503,441282327,16531729,823378840,143542612]));
    assert_eq!(1, Solution::find_radius(vec![1,6], vec![1,5]));
    assert_eq!(1, Solution::find_radius(vec![1,5], vec![1,6]));
    assert_eq!(1, Solution::find_radius(vec![1,2,3], vec![2]));
    assert_eq!(1, Solution::find_radius(vec![1,2,3,4], vec![1,4]));
    assert_eq!(3, Solution::find_radius(vec![1,5], vec![2]));
}