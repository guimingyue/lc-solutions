use std::collections::{HashSet, VecDeque};
use std::io::Write;
use crate::lc::Solution;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        const CHARS: [u8; 4] = ['A' as u8, 'C' as u8, 'G' as u8, 'T' as u8];
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let start_bytes = start.into_bytes();
        visited.insert(start_bytes.clone());
        queue.push_back((start_bytes, 0));


        let end_bytes = end.into_bytes();
        while let Some((bytes, level)) = queue.pop_front() {
            for i in 0..bytes.len() {
                for b in &CHARS {
                    let mut vec = vec![];
                    vec.write_all(&bytes[0..i]);
                    vec.push(*b);
                    vec.write_all(&bytes[i + 1..bytes.len()]);
                    if visited.contains(&vec) {
                        continue;
                    }
                    let str = String::from_utf8_lossy(vec.as_slice()).to_string();
                    if !bank.contains(&str) {
                        continue;
                    }

                    if end_bytes.eq(&vec) {
                        return level + 1;
                    }
                    visited.insert(vec.clone());
                    queue.push_back((vec, level + 1));
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    fn test(start: &str, end: &str, bank: Vec<&str>) -> i32 {
        Solution::min_mutation(start.to_string(), end.to_string(), bank.iter().map(|s|s.to_string()).collect())
    }
    assert_eq!(1, test("AACCGGTT", "AACCGGTA", vec!["AACCGGTA"]));
    assert_eq!(-1, test("AACCGGTT", "AACCGGAA", vec!["AACCGGTA", "AAACGGTA"]));
    assert_eq!(2, test("AACCGGTT", "AAACGGTA", vec!["AACCGGTA","AACCGCTA","AAACGGTA"]));
    assert_eq!(3, test("AAAAACCC", "AACCCCCC", vec!["AAAACCCC","AAACCCCC","AACCCCCC"]));
}