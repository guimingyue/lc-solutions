use crate::lc::Solution;

/// #String, char, Slice
impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        if num.len() < 3 {
            return false;
        }

        fn add_str(first_start: usize, first_end: usize, second_start: usize, second_end: usize, nums: &[u8]) -> String {
            let mut carry = 0;
            let mut i = first_end as i32;
            let mut j = second_end as i32;
            let mut add_sum = String::new();
            while j >= second_start as i32 || i >= first_start as i32 {
                let v1 = if i >= first_start as i32 {
                    (nums[i as usize] as char).to_digit(10).unwrap()
                } else {
                    0
                };
                let v2 = if j >= second_start as i32 {
                    (nums[j as usize] as char).to_digit(10).unwrap()
                } else {
                    0
                };
                let sum = v1 + v2 + carry;
                let v = sum % 10;
                add_sum.insert(0, char::from_digit(v, 10).unwrap());
                carry = sum / 10;
                i -= 1;
                j -= 1;
            }
            if carry > 0 {
                add_sum.insert(0, char::from_digit(carry, 10).unwrap())
            }
            add_sum
        }

        fn valid(mut second_start: usize, mut second_end: usize, nums: &[u8]) -> bool {
            if nums[second_start] == '0' as u8 && second_end > second_start {
                return false;
            }
            let mut first_start = 0;
            while second_end < nums.len() {
                let sum = add_str(first_start, second_start-1, second_start, second_end, nums);
                let (third_start, third_end) = (second_end + 1, second_end + sum.len());
                if third_end >= nums.len() {
                    return false;
                }
                if sum.as_bytes().eq(&nums[third_start..=third_end]) {
                    first_start = second_start;
                    second_start = third_start;
                    second_end = third_end;
                } else {
                    break;
                }
                if second_end == nums.len() -1 {
                    return true;
                }
            }
            false
        }

        let nums = num.as_bytes();
        for second_start in 1..nums.len()-1 {
            for second_end in second_start..nums.len() {
                if valid(second_start, second_end, nums) {
                    return true;
                }
            }
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::is_additive_number("101".to_string()));
    assert!(!Solution::is_additive_number("199001200".to_string()));
    assert!(Solution::is_additive_number("12214".to_string()));
    assert!(Solution::is_additive_number("199100199".to_string()));
    assert!(Solution::is_additive_number("123".to_string()));
    assert!(!Solution::is_additive_number("124".to_string()));
    assert!(Solution::is_additive_number("112358".to_string()));
    assert!(!Solution::is_additive_number("12".to_string()));
}