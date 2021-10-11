struct Solution;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        let vec = vec!["", "Thousand", "Million", "Billion"];
        let mut n = num;
        let mut res = String::new();
        let mut idx = 0;
        while n > 0 {
            let mod_v = n % 1000;
            if mod_v > 0 {
                let mut temp = String::new();
                if idx > 0 {
                    temp.insert(0, ' ');
                }
                temp.insert_str(0, vec[idx]);
                if vec[idx].len() > 0 {
                    temp.insert(0, ' ');
                }
                temp.insert_str(0, Solution::convert_hundred(mod_v).as_str());
                res.insert_str(0, temp.as_str());
            }
            n = n / 1000;
            idx += 1;
        }
        res.trim().to_string()
    }

    fn convert_hundred(num: i32) -> String {
        let mut res = String::new();
        let mut n = num;
        let n_hundred = n / 100;
        if n_hundred > 0 {
            res.push_str(Solution::convert_unit(n_hundred));
            res.push_str(" Hundred");
            n = n % 100;
            if n > 0 {
                res.push(' ');
            }
        }
        res.push_str(Solution::convert_decade(n).as_str());
        res
    }

    fn convert_decade(num: i32) -> String {
        match num {
            10 => "Ten".to_string(),
            11 => "Eleven".to_string(),
            12 => "Twelve".to_string(),
            13 => "Thirteen".to_string(),
            14 => "Fourteen".to_string(),
            15 => "Fifteen".to_string(),
            16 => "Sixteen".to_string(),
            17 => "Seventeen".to_string(),
            18 => "Eighteen".to_string(),
            19 => "Nineteen".to_string(),
            20 => "Twenty".to_string(),
            30 => "Thirty".to_string(),
            40 => "Forty".to_string(),
            50 => "Fifty".to_string(),
            60 => "Sixty".to_string(),
            70 => "Seventy".to_string(),
            80 => "Eighty".to_string(),
            90 => "Ninety".to_string(),
            _ => {
                let mut res = String::new();
                let mut n = num;
                if n / 10 > 0 {
                    res.push_str(Solution::convert_decade(n - n % 10).as_str());
                    if n % 10 > 0 {
                        res.push(' ');
                    }
                }
                if n % 10 > 0 {
                    res.push_str(Solution::convert_unit(n % 10));
                }
                res
            }
        }
    }

    fn convert_unit<'a>(num : i32) -> &'a str {
        match num {
            1 => "One",
            2 => "Two",
            3 => "Three",
            4 => "Four",
            5 => "Five",
            6 => "Six",
            7 => "Seven",
            8 => "Eight",
            9 => "Nine",
            _ => ""
        }
    }
}

#[test]
fn test() {
    assert_eq!("One", Solution::convert_unit(1));
    assert_eq!("Eight", Solution::convert_unit(8));
    assert_eq!("Twenty Eight", Solution::convert_decade(28));
    assert_eq!("Twenty", Solution::convert_decade(20));
    assert_eq!("One Hundred", Solution::convert_hundred(100));
    assert_eq!("One Hundred One", Solution::convert_hundred(101));
    assert_eq!("One Hundred Eleven", Solution::convert_hundred(111));
    assert_eq!("One Hundred Seventeen", Solution::convert_hundred(117));
    assert_eq!("One Hundred Twenty One", Solution::convert_hundred(121));
    assert_eq!("One Hundred Twenty", Solution::convert_hundred(120));
    
    assert_eq!("Zero", Solution::number_to_words(0));
    assert_eq!("One Million", Solution::number_to_words(1000000));
    assert_eq!("One Hundred Twenty", Solution::number_to_words(120));
    assert_eq!("One Hundred", Solution::number_to_words(100));
    assert_eq!("One Thousand", Solution::number_to_words(1000));
    assert_eq!("One Hundred Twenty Three", Solution::number_to_words(123));
    assert_eq!("Twelve Thousand Three Hundred Forty Five", Solution::number_to_words(12345));
    assert_eq!("One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven", Solution::number_to_words(1234567));
    assert_eq!("One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One", 
               Solution::number_to_words(1234567891));
}