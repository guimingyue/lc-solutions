
pub struct Solution {
    
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let str = s.get(..);
        match str {
            None => 0i32,
            Some(ss) => {
                let mut sum = 0;
                let mut pre = ' ';
                for c in ss.chars() {
                    let mut cNum = 0;
                    match c {
                        'I' => cNum = 1,
                        'V' => {
                            if pre == 'I' {
                                sum -= 1;
                                cNum = 4;
                            } else {
                                cNum = 5
                            }
                        },
                        'X' => {
                            if pre == 'I' {
                                sum -= 1;
                                cNum = 9;
                            } else {
                                cNum = 10;
                            }
                        },
                        'L' => {
                            if pre == 'X' {
                                sum -= 10;
                                cNum = 40;
                            } else {
                                cNum = 50;
                            }
                        },
                        'C' => {
                            if pre == 'X' {
                                sum -= 10;
                                cNum = 90;
                            } else {
                                cNum = 100;
                            }
                        },
                        'D' => {
                            if pre == 'C' {
                                sum -= 100;
                                cNum = 400;
                            } else {
                                cNum = 500;
                            }
                        },
                        'M' => {
                            if pre == 'C' {
                                sum -= 100;
                                cNum = 900;
                            } else {
                                cNum = 1000;
                            }
                        }
                        _ => {}
                    }
                    sum += cNum;
                    pre = c;
                }
                sum
            }
        }
    }
}