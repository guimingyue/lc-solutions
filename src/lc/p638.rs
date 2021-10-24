
struct Solution {}

/// #Vec, Reference, std::cmp::min
impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        Solution::dfs(&price, &special, &needs)
    }

    fn dfs(price: &Vec<i32>, special: &Vec<Vec<i32>>, needs: &Vec<i32>) -> i32 {
        let mut min_val = 0;
        for idx in 0..needs.len() {
            min_val += price[idx] * needs[idx];
        }

        for idx in 0..special.len() {
            let idx_special = &special[idx];
            let mut next_needs = vec![];
            for nidx in 0..needs.len() {
                if idx_special[nidx] > needs[nidx] {
                    break;
                }
                next_needs.push(needs[nidx] - idx_special[nidx])
            }
            if next_needs.len() == needs.len() {
                min_val = std::cmp::min(min_val, Solution::dfs(price, special, &next_needs) + idx_special[needs.len()])
            }
        }
        min_val
    }
}

#[test]
fn test() {
    assert_eq!(14, Solution::shopping_offers(vec![2, 5], vec![vec![3,0,5], vec![1,2,10]], vec![3, 2]));
    assert_eq!(11, Solution::shopping_offers(vec![2,3,4], vec![vec![1,1,0,4], vec![2,2,1,9]], vec![1,2,1]));
}