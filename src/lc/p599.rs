use std::collections::HashMap;
use crate::lc::Solution;

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        let mut res = vec![];
        for (idx, str) in list1.iter().enumerate() {
            map.insert(str.as_str(), idx);
        }

        let mut sum = u32::MAX as usize;
        for (idx, str) in list2.iter().enumerate() {
            if let Some(i) = map.get(str.as_str()) {
                let temp_sum = *i + idx;
                if temp_sum > sum {
                    continue;
                } else if temp_sum < sum {
                    sum = temp_sum;
                    res.clear();
                }
                res.push(str.clone());
            }
        }

        res
    }
}

#[test]
fn test() {
    assert_eq!(vec!["Shogun"], Solution::find_restaurant(vec!["Shogun".to_string(),"Tapioca Express".to_string(),"Burger King".to_string(),"KFC".to_string()],
                                                         vec!["Piatti".to_string(),"The Grill at Torrey Pines".to_string(),"Hungry Hunter Steakhouse".to_string(),"Shogun".to_string()]));

    assert_eq!(vec!["Shogun"], Solution::find_restaurant(vec!["Shogun".to_string(),"Tapioca Express".to_string(),"Burger King".to_string(),"KFC".to_string()],
                                                         vec!["KFC".to_string(),"Shogun".to_string(),"Burger King".to_string()]));
}