use crate::lc::Solution;

impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut line_num = 0;
        let mut line_width = 0;
        for ch in s.chars() {
            let width = widths[ch as usize - 'a' as usize];
            if line_width + width > 100 {
                line_width = width;
                line_num += 1;
            } else {
                line_width += width;
            }
        }
        if line_width > 0 {
            line_num += 1;
        }
        vec![line_num, line_width]
    }
}

#[test]
fn test() {
    assert_eq!(vec![1, 100], Solution::number_of_lines(vec![10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10], "abcdefghij".to_string()));
    assert_eq!(vec![3, 60], Solution::number_of_lines(vec![10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10], "abcdefghijklmnopqrstuvwxyz".to_string()));
    assert_eq!(vec![2, 4], Solution::number_of_lines(vec![4,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10], "bbbcccdddaaa".to_string()));
}