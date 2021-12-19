use crate::lc::Solution;

/// #Vec
impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut board = board;
        let mut res = 0;
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == '.' {
                    continue;
                }
                res += 1;
                if j + 1 < board[i].len() && board[i][j+1] == 'X' {
                    for k in (j + 1)..board[i].len() {
                        if board[i][k] == '.' {
                            break
                        }
                        board[i][k] = '.';
                    }
                }
                if i + 1 < board.len() && board[i+1][j] == 'X' {
                    for k in (i+1)..board.len() {
                        if board[k][j] == '.' {
                            break;
                        }
                        board[k][j] = '.';
                    }
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(5, Solution::count_battleships(vec![vec!['.','X','.','.','X'],vec!['.','X','.','.','X'],vec!['.','.','.','.','X'],vec!['X','.','X','X','.'],vec!['X','.','.','.','X']]));
    assert_eq!(0, Solution::count_battleships(vec![vec!['.']]));
    assert_eq!(2, Solution::count_battleships(vec![vec!['X','.','.','X'],vec!['.','.','.','X'],vec!['.','.','.','X']]));
    assert_eq!(2, Solution::count_battleships(vec![vec!['X','.','.','X'],vec!['.','.','.','X'],vec!['.','.','.','X'],vec!['.','.','.','.']]));
    assert_eq!(3, Solution::count_battleships(vec![vec!['X','.','.','X'],vec!['.','.','.','X'],vec!['X','X','.','X'],vec!['.','.','.','.']]));
}