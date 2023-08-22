#![allow(dead_code)]
/*
130. 被围绕的区域
中等
1K
相关企业
给你一个 m x n 的矩阵 board ，由若干字符 'X' 和 'O' ，找到所有被 'X' 围绕的区域，并将这些区域里所有的 'O' 用 'X' 填充。


示例 1：


输入：board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
输出：[["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
解释：被围绕的区间不会存在于边界上，换句话说，任何边界上的 'O' 都不会被填充为 'X'。 任何不在边界上，或不与边界上的 'O' 相连的 'O' 最终都会被填充为 'X'。如果两个元素在水平或垂直方向相邻，则称它们是“相连”的。
示例 2：

输入：board = [["X"]]
输出：[["X"]]


提示：

m == board.length
n == board[i].length
1 <= m, n <= 200
board[i][j] 为 'X' 或 'O'
 */
struct Solution;
/*
只有与边界上的 'O' 连通的 'O' 最终才填充为 'O'
否则都将被填充为 'X'

遍历边界上的 'O', 深度搜索连通的'O', 并将它们标记，比如标记为'#'
搜索完成后，遍历完整的矩阵，将所有的 '#' 填充为 'O'，其它任何值填充为 'X'
 */
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (m, n) = (board.len(), board[0].len());
        // 上下边界
        for j in 0..n {
            dfs(board, 0, j as i32, m as i32, n as i32);
            if m > 1 {
                dfs(board, (m - 1) as i32, j as i32, m as i32, n as i32);
            }
        }
        // 左右边界
        for i in 0..m {
            dfs(board, i as i32, 0, m as i32, n as i32);
            if n > 1 {
                dfs(board, i as i32, (n - 1) as i32, m as i32, n as i32);
            }
        }
        // 重新填充矩阵
        for i in 0..m {
            for j in 0..n {
                if board[i][j] == '#' {
                    board[i][j] = 'O';
                }else {
                    board[i][j] = 'X';
                }
            }
        }
    }
}

fn dfs(board: &mut Vec<Vec<char>>, i: i32, j: i32, m: i32, n: i32) {
    if i < 0 || i >= m || j < 0 || j >= n {
        return;
    }
    let x = i as usize;
    let y = j as usize;
    if board[x][y] == 'O' {
        board[x][y] = '#';
        dfs(board, i - 1, j, m, n);
        dfs(board, i + 1, j, m, n);
        dfs(board, i, j - 1, m, n);
        dfs(board, i, j + 1, m, n);
    }
}

#[cfg(test)]
mod test {
    use crate::arr::solve_round::Solution;

    #[test]
    pub fn t1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        println!("{:?}", board);
    }

    #[test]
    pub fn t2() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'O'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'O', 'X'],
        ];
        Solution::solve(&mut board);
        println!("{:?}", board);
    }
}