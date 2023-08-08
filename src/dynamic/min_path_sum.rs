#![allow(dead_code)]
/*
64. 最小路径和
中等
1.6K
相关企业
给定一个包含非负整数的 m x n 网格 grid ，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。

说明：每次只能向下或者向右移动一步。



示例 1：


输入：grid = [[1,3,1],[1,5,1],[4,2,1]]
输出：7
解释：因为路径 1→3→1→1→1 的总和最小。
示例 2：

输入：grid = [[1,2,3],[4,5,6]]
输出：12


提示：

m == grid.length
n == grid[i].length
1 <= m, n <= 200
0 <= grid[i][j] <= 200
 */
struct Solution;
/*
动态规划
dp[i][j] 表示 到达(i,j)的最小路径和

状态转移方程：
dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i][j]

初始状态
dp[0][0] = grid[0][0]
dp[i][0] = dp[i-1][0] + grid[i][0]
dp[0][j] = dp[0][j-1] + grid[0][j]

 */
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; n]; m];
        // 初始化状态
        dp[0][0] = grid[0][0];
        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + grid[i][0]
        }
        for j in 1..n {
            dp[0][j] = dp[0][j - 1] + grid[0][j]
        }
        // 计算状态转移
        for i in 1..m {
            for j in 1..n {
                dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }
        dp[m - 1][n - 1]
    }
}


#[cfg(test)]
mod test {
    use crate::dynamic::min_path_sum::Solution;

    #[test]
    pub fn t1() {
        let vec = vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1],
        ];
        assert_eq!(Solution::min_path_sum(vec), 7);
    }
}

