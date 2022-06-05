use crate::lc::Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        fn read_word(bytes: &[u8], idx: usize) -> (usize, bool) {
            let mut i = idx;
            let mut is_file = false;
            while i < bytes.len() && bytes[i] != '\n' as u8 {
                if bytes[i] == '.' as u8 {
                    is_file = true;
                }
                i += 1;
            }
            (i, is_file)
        }
        fn read_nt(bytes: &[u8], idx: usize) -> (usize, usize) {
            let mut i = idx;
            let mut t_count = 0;
            if bytes[i] != '\n' as u8 {
                return (i, t_count)
            }
            i += 1;
            while i < bytes.len() && bytes[i] == '\t' as u8 {
                t_count += 1;
                i += 1;
            }
            (i, t_count)
        }
        let mut stack: Vec<(usize, usize)> = vec![];
        let bytes = input.as_bytes();
        let mut i = 0;
        let mut max_len = 0;
        while i < bytes.len() {
            let (end_idx, t_count) = read_nt(bytes, i);
            i = end_idx;
            let (word_end_idx, is_file) = read_word(bytes, i);
            while !stack.is_empty() {
                if stack.last().unwrap().0 >= t_count {
                    stack.pop();
                } else {
                    break;
                }
            }
            let dir_len = if stack.is_empty() {
                0
            } else {
                stack.last().unwrap().1
            };
            if is_file {
                let len = dir_len + word_end_idx - i + stack.len();
                if len > max_len {
                    max_len = len;
                }
            } else {
                stack.push((t_count, dir_len + word_end_idx - i));
            }
            i = word_end_idx;
        }

        max_len as i32
    }
}

#[test]
fn test() {
    assert_eq!(16, Solution::length_longest_path("dir\n        file.txt".to_string()));
    assert_eq!(0, Solution::length_longest_path("a".to_string()));
    assert_eq!(12, Solution::length_longest_path("file1.txt\nfile2.txt\nlongfile.txt".to_string()));
    assert_eq!(20, Solution::length_longest_path("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext".to_string()));
    assert_eq!(32, Solution::length_longest_path("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext".to_string()));
}