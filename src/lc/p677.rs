

/// #HashMap, PartialEq, Eq, Default
struct MapSum {
    root: Trie
}

#[derive(PartialEq, Eq, Default, Debug)]
struct Trie {
    children: std::collections::HashMap<char, Trie>,
    end: bool,
    val: i32
}


impl Trie {

    fn insert(&mut self, str: &str, val: i32) {
        let mut p = self;
        for ch in str.chars() {
            p = p.children.entry(ch).or_default()
        }
        p.end = true;
        p.val = val;
    }

    fn prefix_sum(&self, str: &str) -> i32 {
        if !str.is_empty() {
            let ch = str.chars().next().unwrap();
            if let Some(t) = self.children.get(&ch) {
                t.prefix_sum(&str[1..])
            } else {
                0
            }
        } else {
            let mut sum = 0;
            if self.end {
                sum += self.val;
            }
            for (_, t) in self.children.iter() {
                sum += t.prefix_sum(String::new().as_str());
            }
            sum
        }
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MapSum {

    fn new() -> Self {
        MapSum {
            root: Trie::default()
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        if key.is_empty() {
            return;
        }
        self.root.insert(key.as_str(), val);
    }

    fn sum(&self, prefix: String) -> i32 {
        self.root.prefix_sum(prefix.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut mapSum = MapSum::new();
        mapSum.insert("ap".to_string(), 1);
        assert_eq!(1, mapSum.sum("ap".to_string()));
    }

    #[test]
    fn test2() {
        let mut mapSum = MapSum::new();
        mapSum.insert("apple".to_string(), 3);
        assert_eq!(3, mapSum.sum("ap".to_string()));
        mapSum.insert("app".to_string(), 2);
        assert_eq!(5, mapSum.sum("ap".to_string()));
        mapSum.insert("ap".to_string(), 1);
        assert_eq!(6, mapSum.sum("ap".to_string()));
    }

    #[test]
    fn test3() {
        let mut mapSum = MapSum::new();
        mapSum.insert("apx".to_string(), 1);
        assert_eq!(0, mapSum.sum("apy".to_string()));
    }

    #[test]
    fn test4() {
        let mut mapSum = MapSum::new();
        mapSum.insert("aa".to_string(), 3);
        assert_eq!(3, mapSum.sum("a".to_string()));
        mapSum.insert("aa".to_string(), 2);
        assert_eq!(2, mapSum.sum("a".to_string()));
        assert_eq!(2, mapSum.sum("aa".to_string()));
        mapSum.insert("aaa".to_string(), 3);
        assert_eq!(3, mapSum.sum("aaa".to_string()));
    }
}