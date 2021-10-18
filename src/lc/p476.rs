struct Solution {}

/// >>, <<
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut shift = 1;
        let mut n = num;
        let mut res = Solution::nega(n & 1);
        while n >> 1 > 0 {
            n = n >> 1;
            res = res | (Solution::nega(n & 1) << shift);
            shift += 1;
        }
        res
    }

    fn nega(n: i32) -> i32 {
        match n {
            1 => 0,
            _ => 1
        }
    }
}

#[test]
fn test() {
    assert_eq!(2, Solution::find_complement(5));
    assert_eq!(0, Solution::find_complement(1));
    assert_eq!(1, Solution::find_complement(2));
    assert_eq!(0, Solution::find_complement(3));
    assert_eq!(0, Solution::find_complement(7));
    assert_eq!(5, Solution::find_complement(10));
}