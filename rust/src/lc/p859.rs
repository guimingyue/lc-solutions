use crate::lc::Solution;

/// #String
impl Solution {
    pub fn buddy_strings_on2(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let mut sv = s.into_bytes();
        let mut goal_v = goal.into_bytes();
        for i in 0..sv.len() {
            for j in i+1..sv.len() {
                sv.swap(i, j);
                if sv.eq(&goal_v) {
                    return true;
                }
                sv.swap(i, j);
            }
        }
        false
    }

    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let mut sv = s.into_bytes();
        let mut goal_v = goal.into_bytes();
        let mut aux = vec![0; 26];
        let mut contain_dup = false;
        let mut diff_num = 0;
        for i in 0..sv.len() {
            if sv[i] != goal_v[i] {
                diff_num += 1;
            }
            let idx = (sv[i] - 'a' as u8) as usize;
            aux[idx] += 1;
            if aux[idx] > 1 {
                contain_dup = true;
            }
        }

        for i in 0..goal_v.len() {
            let idx = (goal_v[i] - 'a' as u8) as usize;
            aux[idx] -= 1;
            if aux[idx] < 0 {
                return false;
            }
        }

        if diff_num == 2 {
            return true;
        }
        if diff_num == 0 && contain_dup {
            return true;
        }

        false
    }
}

#[test]
fn test() {
    assert!(!Solution::buddy_strings("a".to_string(), "a".to_string()));
    assert!(Solution::buddy_strings("ab".to_string(), "ba".to_string()));
    assert!(!Solution::buddy_strings("ab".to_string(), "ab".to_string()));
    assert!(!Solution::buddy_strings("abc".to_string(), "abc".to_string()));
    assert!(Solution::buddy_strings("abc".to_string(), "acb".to_string()));
    assert!(Solution::buddy_strings("aa".to_string(), "aa".to_string()));
    assert!(Solution::buddy_strings("aaaaaaabc".to_string(), "aaaaaaacb".to_string()));
}