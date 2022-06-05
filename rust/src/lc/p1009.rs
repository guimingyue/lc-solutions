struct Solution {
    
}

/// >>, <<
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut shift = 1;
        let mut num = n;
        let mut res = Solution::nega(num & 1);
        while num >> 1 > 0 {
            num = num >> 1;
            res = res | (Solution::nega(num & 1) << shift);
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
    assert_eq!(2, Solution::bitwise_complement(5));
    assert_eq!(0, Solution::bitwise_complement(1));
    assert_eq!(1, Solution::bitwise_complement(2));
    assert_eq!(0, Solution::bitwise_complement(3));
    assert_eq!(0, Solution::bitwise_complement(7));
    assert_eq!(5, Solution::bitwise_complement(10));
}