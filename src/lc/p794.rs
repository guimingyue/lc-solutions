use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        fn win(b: &Vec<Vec<u8>>, x: u8) -> bool {
            for i in 0..b.len() {
                if x == b[i][0] && x == b[i][1] && x == b[i][2] {
                    return true;
                }
                if x == b[0][i] && x == b[1][i] && x == b[2][i] {
                    return true;
                }
            }
            if x == b[0][0] && x == b[1][1] && x == b[2][2] {
                return true;
            }
            if x == b[0][2] && x == b[1][1] && x == b[2][0] {
                return true;
            }
            false
        }
        let b:Vec<Vec<u8>> = board.iter().map(|str|str.clone().into_bytes()).collect();
        let (mut x_count, mut o_count) = (0, 0);
        for i in 0..b.len() {
            for j in 0..b[i].len() {
                if b[i][j] == 'X' as u8 {
                    x_count += 1;
                }
                if b[i][j] == 'O' as u8 {
                    o_count += 1;
                }
            }
        }

        if x_count != o_count && x_count != o_count + 1 {
            return false;
        }

        if win(&b, 'X' as u8) && x_count != o_count + 1 {
            return false;
        }

        if win(&b, 'O' as u8) && x_count != o_count {
            return false;
        }

        true
    }
}

#[test]
fn test() {
    assert!(!Solution::valid_tic_tac_toe(vec!["O  ".to_string(),"   ".to_string(),"   ".to_string()]));
    assert!(!Solution::valid_tic_tac_toe(vec!["XOX".to_string()," X ".to_string(),"   ".to_string()]));
    assert!(!Solution::valid_tic_tac_toe(vec!["XXX".to_string(),"   ".to_string(),"OOO".to_string()]));
    assert!(Solution::valid_tic_tac_toe(vec!["XOX".to_string(),"O O".to_string(),"XOX".to_string()]));
}