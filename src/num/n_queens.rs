#![allow(dead_code)]
/*
51. N 皇后
困难
1.8K
相关企业
按照国际象棋的规则，皇后可以攻击与之处在同一行或同一列或同一斜线上的棋子。

n 皇后问题 研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。

给你一个整数 n ，返回所有不同的 n 皇后问题 的解决方案。

每一种解法包含一个不同的 n 皇后问题 的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。



示例 1：


输入：n = 4
输出：[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
解释：如上图所示，4 皇后问题存在两个不同的解法。
示例 2：

输入：n = 1
输出：[["Q"]]


提示：

1 <= n <= 9

52. N 皇后 II
给你一个整数 n ，返回 n 皇后问题 不同的解决方案的数量。
 */


struct Solution;
/*
每行、每列都只有一个皇后
每条斜线只能有一个皇后，对每个位置来说，斜线有两个方向

#
  #
    #
斜线1：j-i 是一个定值

      #
   #
#
斜线1：i+j 是一个定值

 */
use std::collections::HashSet;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut ans = Vec::new();
        let mut column = HashSet::new();
        let mut diag1 = HashSet::new();
        let mut diag2 = HashSet::new();
        back_track(n, &mut Vec::new(), &mut column, &mut diag1, &mut diag2, &mut ans);
        ans
    }


    pub fn total_n_queens(n: i32) -> i32 {
        Self::solve_n_queens(n).len() as i32
    }
}

fn back_track(n: i32, state: &mut Vec<i32>, column: &mut HashSet<i32>, diag1: &mut HashSet<i32>,
              diag2: &mut HashSet<i32>, ans: &mut Vec<Vec<String>>) {
    let i = state.len() as i32;
    if i == n {
        ans.push(gen_ans(state));
        return;
    }
    // 要在第i行放置一个皇后 枚举回溯可能的列
    for j in 0..n {
        if column.contains(&j) {
            continue;
        }
        if diag1.contains(&(j - i)) {
            continue;
        }
        if diag2.contains(&(j + i)) {
            continue;
        }
        // 放置一个皇后到(i,j)
        state.push(j);
        // 标记这个位置
        column.insert(j);
        diag1.insert(j - i);
        diag2.insert(j + i);
        // 回溯
        back_track(n, state, column, diag1, diag2, ans);
        // 撤销选择
        state.pop();
        column.remove(&j);
        diag1.remove(&(j - i));
        diag2.remove(&(j + i));
    }
}

fn gen_ans(state: &mut Vec<i32>) -> Vec<String> {
    let n = state.len();
    state.iter()
        .map(|&x| {
            let mut row = std::iter::repeat('.').take(n-1).collect::<String>();
            row.insert(x as usize, 'Q');
            row
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::num::n_queens::Solution;

    #[test]
    fn t1() {
        let ans = Solution::solve_n_queens(4);
        println!("{:?}", ans);
    }

}