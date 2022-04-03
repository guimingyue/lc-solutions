use crate::lc::Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        if target >= letters[letters.len() - 1] {
            return letters[0];
        }
        let (mut left, mut right) = (0, letters.len() - 1);
        while left < right {
            let mid = (right - left) / 2 + left;
            if letters[mid] > target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        letters[left]
    }
}

#[test]
fn test() {
    assert_eq!('c', Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'j'));
    assert_eq!('c', Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'a'));
    assert_eq!('f', Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'c'));
    assert_eq!('f', Solution::next_greatest_letter(vec!['c', 'f', 'j'], 'd'));
}