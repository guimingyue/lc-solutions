use crate::lc::Solution;

/// reference from https://leetcode-cn.com/problems/zuma-game/solution/gong-shui-san-xie-yi-ti-shuang-jie-sou-s-3ftb/1220635/
use std::collections::HashSet;

/// #HashSet
impl Solution {
    pub fn find_min_step(board: String, hand: String) -> i32 {
        // 辅助函数: 将手中的球的信息的字符串表示形式转换为数组信息
        fn parse_hand(hand: &[u8]) -> Vec<u8> {
            // 因为只有五种球, 这就非常的方便
            let mut res = vec![0; 5];
            hand.iter().for_each(|&ch| match ch {
                b'R' => res[0] += 1,
                b'Y' => res[1] += 1,
                b'B' => res[2] += 1,
                b'G' => res[3] += 1,
                _ => res[4] += 1,
            });

            res
        }

        // 辅助函数: 检查当前状态能否最终消除完
        fn can_clear(board: &Vec<u8>, hand: &Vec<u8>) -> bool {
            let mut ball_cnt = parse_hand(&board[..]); // 借用一下解析手中字符串信息的函数
            // 剪枝: 如果这个球出现在了桌子上， 且桌面上的球 + 手中的球 后 还有数量小于2的球, 那我们就无法清空桌面
            !(0..ball_cnt.len()).any(|i| ball_cnt[i] > 0 && 0 < ball_cnt[i] + hand[i] && ball_cnt[i] + hand[i] < 3)
        }

        // 辅助函数: 依据当前状态, 产生所有可能的下一状态
        fn gen_next_status(board: &Vec<u8>, hand: &Vec<u8>) -> HashSet<(Vec<u8>, Vec<u8>)> {
            let ball = [b'R', b'Y', b'B', b'G', b'W'];
            let mut next_status = HashSet::new();
            // 我们考虑使用不同颜色的球
            (0..hand.len()).filter(|ball_type| hand[*ball_type] > 0).for_each(|ball_type| {
                let mut next_hand = hand.to_vec();
                next_hand[ball_type] -= 1;

                for insert_idx in 0..=board.len() {
                    // 考虑不同的插空位置
                    next_status.insert((gen_helper(board, insert_idx, ball[ball_type]), next_hand.to_vec()));
                }
            });

            next_status
        }

        // 辅助函数: 考虑使用特定的球插入特定的位置, 来获得下一状态
        fn gen_helper(board: &Vec<u8>, insert_idx: usize, ball_type: u8) -> Vec<u8> {
            // 这里主要是要考虑消除的情况, 我们可以很好地用栈来模拟
            let mut board = board.to_vec();
            board.insert(insert_idx, ball_type);

            let mut res = Vec::new();
            board.iter().for_each(|&ch| {
                let n = res.len();
                if n >= 3 && ch != res[n-1] && res[n-1] == res[n-2] && res[n-1] == res[n-3] {
                    let delete_ch = res[n-1];
                    while *res.last().unwrap_or(&b'#') == delete_ch {
                        res.pop();
                    }
                }

                res.push(ch);
            });

            let n = res.len();
            if n >= 3 && res[n-1] == res[n-2] && res[n-1] == res[n-3] {
                let delete_ch = res[n-1];
                while *res.last().unwrap_or(&b'#') == delete_ch {
                    res.pop();
                }
            }

            res
        }

        let mut step = 0;
        // 状态是什么?
        // 状态就是 当前一排球的排列 和 手中剩余球的数量信息
        let mut curr_status = HashSet::new();
        // 因为可以从手中选出 任意一颗, 我们可以不必在意顺序, 只需要统计其数量信息即可
        curr_status.insert((board.as_bytes().to_vec(), parse_hand(hand.as_bytes())));

        // 然后我们就开始广搜
        while !curr_status.is_empty() {
            step += 1;
            let mut next_status = HashSet::new();

            for (board_info, hand_info) in curr_status.drain() {
                // 那状态要怎么转移呢?
                // 我们可以尝试使用手里不同的球, 然后插到不同的位置上去, 就可以产生下一步的状态了
                // 剪枝: 如果这个球出现在了桌子上， 且桌面上的球 + 手中的球 后 还有数量小于2的球, 那我们就无法清空桌面
                if !can_clear(&board_info, &hand_info) {
                    continue;
                }

                // 我们要使用当前状态来产生下一步所有可能的状态
                // next_st = (next_board, next_hand)
                for next_st in gen_next_status(&board_info, &hand_info).drain() {
                    if next_st.0.is_empty() {
                        // 成功地消除完了
                        return step;
                    }

                    // 可以再减一下枝
                    if !can_clear(&next_st.0, &next_st.1) {
                        continue;
                    }

                    next_status.insert(next_st);
                }
            }

            curr_status = next_status;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_step() {
        assert_eq!(
            Solution::find_min_step("WRRBBW".to_owned(), "RB".to_owned()),
            -1
        );
    }

    #[test]
    fn test_find_min_step_02() {
        assert_eq!(
            Solution::find_min_step("WWRRBBWW".to_owned(), "WRBRW".to_owned()),
            2
        );
    }

    #[test]
    fn test_find_min_step_03() {
        assert_eq!(
            Solution::find_min_step("G".to_owned(), "GGGGG".to_owned()),
            2
        );
    }

    #[test]
    fn test_find_min_step_04() {
        assert_eq!(
            Solution::find_min_step("RBYYBBRRB".to_owned(), "YRBGB".to_owned()),
            3
        );
    }

    #[test]
    fn test_find_min_step_05() {
        assert_eq!(
            Solution::find_min_step("RRWWRRBBRR".to_owned(), "RRWWRRBBRR".to_owned()),
            2
        );
    }

    #[test]
    fn test_find_min_step_06() {
        assert_eq!(
            Solution::find_min_step("WWBBWBBWW".to_owned(), "BB".to_owned()),
            -1
        );
    }
}