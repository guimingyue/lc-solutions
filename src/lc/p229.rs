struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut elem1 = 0;
        let mut elem2 = 0;
        let mut vote1 = 0;
        let mut vote2 = 0;
        for val in nums.iter() {
            if vote1 > 0 && elem1 == *val {
                vote1 += 1;
            } else if vote2 > 0 && elem2 == *val {
                vote2 += 1;
            } else if vote1 == 0 {
                elem1 = *val;
                vote1 += 1;
            } else if vote2 == 0 {
                elem2 = *val;
                vote2 += 1;
            } else {
                vote1 -= 1;
                vote2 -= 1;
            }
        }

        let mut cnt1 = 0;
        let mut cnt2 = 0;
        for val in nums.iter() {
            if vote1 > 0 && *val == elem1 {
                cnt1 += 1;
            }
            if vote2 > 0 && *val == elem2 {
                cnt2 += 1;
            }
        }
        let mut res = vec![];
        if vote1 > 0 && cnt1 > nums.len() / 3 {
            res.push(elem1);
        }
        if vote2 > 0 && cnt2 > nums.len() / 3 {
            res.push(elem2);
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(vec![3], Solution::majority_element(vec![3,2,3]));
    assert_eq!(vec![1], Solution::majority_element(vec![1]));
    assert_eq!(vec![1,2], Solution::majority_element(vec![1,1,1,3,3,2,2,2]));
}