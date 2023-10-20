#![allow(dead_code)]
/*
329. 矩阵中的最长递增路径
困难
相关标签
相关企业
给定一个 m x n 整数矩阵 matrix ，找出其中 最长递增路径 的长度。

对于每个单元格，你可以往上，下，左，右四个方向移动。 你 不能 在 对角线 方向上移动或移动到 边界外（即不允许环绕）。



示例 1：


输入：matrix = [[9,9,4],[6,6,8],[2,1,1]]
输出：4
解释：最长递增路径为 [1, 2, 6, 9]。
示例 2：


输入：matrix = [[3,4,5],[3,2,6],[2,2,1]]
输出：4
解释：最长递增路径是 [3, 4, 5, 6]。注意不允许在对角线方向上移动。
示例 3：

输入：matrix = [[1]]
输出：1


提示：

m == matrix.length
n == matrix[i].length
1 <= m, n <= 200
0 <= matrix[i][j] <= 231 - 1
 */
use std::collections::VecDeque;

struct Solution;
/*
使用 BFS
搜索前 先找周围更小的数
 */

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![1; n]; m];
        let mut ans = 1;
        for i in 0..m {
            for j in 0..n {
                if dp[i][j] == 1 {
                    let (x, y) = find_min_cell(&matrix, i, j);
                    println!("do bfs start at {:?}", (x, y));
                    // BFS
                    let mut deque = VecDeque::new();
                    deque.push_back((x, y));
                    while let Some((i, j)) = deque.pop_front() {
                        // 右
                        if j + 1 < n && matrix[i][j + 1] > matrix[i][j] && dp[i][j + 1] <= dp[i][j] {
                            dp[i][j + 1] = 1 + dp[i][j];
                            ans = ans.max(dp[i][j + 1]);
                            deque.push_back((i, j + 1));
                        }
                        // 下
                        if i + 1 < m && matrix[i + 1][j] > matrix[i][j] && dp[i + 1][j] <= dp[i][j] {
                            dp[i + 1][j] = 1 + dp[i][j];
                            ans = ans.max(dp[i + 1][j]);
                            deque.push_back((i + 1, j));
                        }
                        // 左
                        if j > 0 && matrix[i][j - 1] > matrix[i][j] && dp[i][j - 1] <= dp[i][j] {
                            dp[i][j - 1] = 1 + dp[i][j];
                            ans = ans.max(dp[i][j - 1]);
                            deque.push_back((i, j - 1));
                        }
                        // 上
                        if i > 0 && matrix[i - 1][j] > matrix[i][j] && dp[i - 1][j] <= dp[i][j] {
                            dp[i - 1][j] = 1 + dp[i][j];
                            ans = ans.max(dp[i - 1][j]);
                            deque.push_back((i - 1, j));
                        }
                    }
                }
            }
        }

        ans
    }
}


fn find_min_cell(matrix: &Vec<Vec<i32>>, i: usize, j: usize) -> (usize, usize) {
    let (m, n) = (matrix.len(), matrix[0].len());
    if j + 1 < n && matrix[i][j + 1] < matrix[i][j] {
        return find_min_cell(matrix, i, j + 1);
    }
    if i + 1 < m && matrix[i + 1][j] < matrix[i][j] {
        return find_min_cell(matrix, i + 1, j);
    }
    if j > 0 && matrix[i][j - 1] < matrix[i][j] {
        return find_min_cell(matrix, i, j - 1);
    }
    if i > 0 && matrix[i - 1][j] < matrix[i][j] {
        return find_min_cell(matrix, i - 1, j);
    }
    (i, j)
}

#[cfg(test)]
mod test {
    use crate::arr::longest_increasing_path::Solution;

    #[test]
    pub fn t1() {
        let matrix = vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]];
        let a = Solution::longest_increasing_path(matrix);
        println!("{}", a);
    }
}