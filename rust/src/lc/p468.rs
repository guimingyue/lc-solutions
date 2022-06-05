use std::str::FromStr;
use crate::lc::Solution;

/// #match
impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        fn is_ipv4(ip: Vec<&str>) -> String {
            for v in ip {
                let parsed = i32::from_str(v).unwrap_or_else(|r|-1);
                if parsed < 0 || parsed > 255 {
                    return "Neither".to_string();
                }

                let bytes = v.as_bytes();
                if bytes[0] == '0' as u8 && bytes.len() > 1 {
                    return "Neither".to_string();
                }
            }
            "IPv4".to_string()
        }

        fn is_ipv6(ip: Vec<&str>) -> String {
            if ip.len() != 8 {
                return "Neither".to_string();
            }
            for v in ip {
                if v.len() < 1 || v.len() > 4 {
                    return "Neither".to_string();
                }
                for ch in v.chars() {
                    match ch {
                        'A'..='F' | 'a'..='f' | '0'..='9' => {},
                        _ => {
                            return "Neither".to_string();
                        }
                    }
                }
            }
            "IPv6".to_string()
        }

        let split: Vec<_> = query_ip.split('.').collect();
        if split.len() == 4 {
            is_ipv4(split)
        } else {
            let split: Vec<_> = query_ip.split(':').collect();
            is_ipv6(split)
        }
    }
}

#[test]
fn test() {
    assert_eq!("Neither", Solution::valid_ip_address("20EE:FGb8:85a3:0:0:8A2E:0370:7334".to_string()));

    assert_eq!("IPv4", Solution::valid_ip_address("192.168.1.1".to_string()));
    assert_eq!("IPv4", Solution::valid_ip_address("192.168.1.0".to_string()));
    assert_eq!("IPv4", Solution::valid_ip_address("172.16.254.1".to_string()));
    assert_eq!("Neither", Solution::valid_ip_address("192.168.01.1".to_string()));
    assert_eq!("Neither", Solution::valid_ip_address("192.168.1.00".to_string()));
    assert_eq!("Neither", Solution::valid_ip_address("192.168@1.1".to_string()));
    assert_eq!("Neither", Solution::valid_ip_address("256.256.256.256".to_string()));

    assert_eq!("IPv6", Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_string()));
    assert_eq!("IPv6", Solution::valid_ip_address("2001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string()));
    assert_eq!("IPv6", Solution::valid_ip_address("2001:db8:85a3:0:0:8A2E:0370:7334".to_string()));
    assert_eq!("Neither", Solution::valid_ip_address("2001:0db8:85a3::8A2E:037j:7334".to_string()));
    assert_eq!("Neither", Solution::valid_ip_address("02001:0db8:85a3:0000:0000:8a2e:0370:7334".to_string()));
}