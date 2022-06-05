use crate::lc::Solution;

/// #Vec, String
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.len() == 0 {
            return vec![];
        }
        fn add_str(a: i32, b: i32, res: &mut Vec<String>) {
            if a == b {
                res.push(a.to_string());
            } else {
                let mut str = String::from(a.to_string());
                str.push_str("->");
                str.push_str(b.to_string().as_str());
                res.push(str);
            }
        }
        let mut res = vec![];
        let mut a = nums[0];
        let mut b = nums[0];
        for i in 1..nums.len() {
            if nums[i] == b + 1 {
                b = nums[i];
            } else {
                add_str(a, b, &mut res);
                a = nums[i];
                b = nums[i]
            }
        }
        add_str(a, b, &mut res);
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec!["0->2","4->5","7"], Solution::summary_ranges(vec![0,1,2,4,5,7]));
    assert_eq!(vec!["0","2->4","6","8->9"], Solution::summary_ranges(vec![0,2,3,4,6,8,9]));
    assert_eq!(Vec::<String>::new(), Solution::summary_ranges(Vec::<i32>::new()));
    assert_eq!(vec!["-1"], Solution::summary_ranges(vec![-1]));
    assert_eq!(vec!["0"], Solution::summary_ranges(vec![0]));
    assert_eq!(vec!["-2147483648->-2147483647", "2147483647"], Solution::summary_ranges(vec![i32::MIN, i32::MIN+1, i32::MAX]));
}