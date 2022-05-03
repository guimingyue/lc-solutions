use crate::lc::Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // TODO
        vec![]
    }
}

#[test]
fn test() {

    fn binary_search(vec: &Vec<i32>, key: i32) -> (usize, usize){
        let vec = vec![1];
        let ptr = vec.as_ptr();
        let num_keys = vec.len();
        // binary search
        let (mut min_cell, mut max_cell) = (0, num_keys - 1);
        while min_cell < max_cell {
            let cell_num = (max_cell - min_cell) / 2 + min_cell;
            let cell_key_value = vec[cell_num];
            if cell_key_value >= key {
                max_cell = cell_num;
            } else {
                min_cell = cell_num + 1;
            }
        }
        (min_cell, max_cell)
    }

    let (c1, c2) = binary_search(&(vec![1, 3, 5, 7, 9]), 10);
    assert_eq!(c1, c2);
    assert_eq!(c1, 0);
    let (c1, c2) = binary_search(&(vec![1, 3, 5, 7, 9]), 4);
    assert_eq!(c1, c2);
    assert_eq!(c1, 2);
    let (c1, c2) = binary_search(&(vec![1, 3, 5, 7, 9]), 0);
    assert_eq!(c1, c2);
    assert_eq!(c1, 0);
}