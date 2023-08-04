#![allow(dead_code)]
/*
36. 有效的数独

中等
1.1K
相关企业
请你判断一个 9 x 9 的数独是否有效。只需要 根据以下规则 ，验证已经填入的数字是否有效即可。

数字 1-9 在每一行只能出现一次。
数字 1-9 在每一列只能出现一次。
数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。（请参考示例图）


注意：

一个有效的数独（部分已被填充）不一定是可解的。
只需要根据以上规则，验证已经填入的数字是否有效即可。
空白格用 '.' 表示。
提示：

board.length == 9
board[i].length == 9
board[i][j] 是一位数字（1-9）或者 '.'
 */
struct Solution;

use std::collections::HashSet;

const CHECK_MARK: [(usize, usize); 9] = [
    (0, 0), (0, 3), (0, 6),
    (3, 0), (3, 3), (3, 6),
    (6, 0), (6, 3), (6, 6),
];

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set = HashSet::new();
        // 行有效
        for row in board.iter() {
            set.clear();
            if row.iter()
                .filter(|&&x| x.is_ascii_digit())
                .any(|&x| set.insert(x) == false) {
                return false;
            }
        }
        // 列有效
        for i in 0..9 {
            set.clear();
            if board.iter()
                .map(|row| row[i])
                .filter(|&x| x.is_ascii_digit())
                .any(|x| set.insert(x) == false) {
                return false;
            }
        }
        // 九宫有效
        let grids = |x: usize, y: usize| {
            vec![
                (x, y), (x, y + 1), (x, y + 2),
                (x + 1, y), (x + 1, y + 1), (x + 1, y + 2),
                (x + 2, y), (x + 2, y + 1), (x + 2, y + 2),
            ]
        };
        for &(x, y) in CHECK_MARK.iter() {
            let cells = grids(x, y);
            set.clear();
            if cells.iter()
                .map(|&(x, y)| board[x][y])
                .filter(|&x| x.is_ascii_digit())
                .any(|x| set.insert(x) == false) {
                return false;
            }
        }
        true
    }
}