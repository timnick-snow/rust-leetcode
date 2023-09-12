#![allow(dead_code)]
/*
221. 最大正方形
中等
相关标签
相关企业
在一个由 '0' 和 '1' 组成的二维矩阵内，找到只包含 '1' 的最大正方形，并返回其面积。



示例 1：


输入：matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
输出：4
示例 2：


输入：matrix = [["0","1"],["1","0"]]
输出：1
示例 3：

输入：matrix = [["0"]]
输出：0


提示：

m == matrix.length
n == matrix[i].length
1 <= m, n <= 300
matrix[i][j] 为 '0' 或 '1'
 */
struct Solution;
/*
动态规划

dp[i][j] = min(dp[i-1][j],dp[i][j-1],dp[i-1][j-1]) + 1
 */
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n]; m];
        let mut ans = 0;
        // 初始状态
        for i in 0..m {
            if matrix[i][0] == '1' {
                dp[i][0] = 1;
                ans = 1;
            }
        }
        for j in 0..n {
            if matrix[0][j] == '1' {
                dp[0][j] = 1;
                ans = 1;
            }
        }
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j]=='1' {
                    let temp = std::cmp::min(dp[i - 1][j], dp[i][j - 1]);
                    dp[i][j] = std::cmp::min(temp, dp[i - 1][j - 1]) + 1;
                    ans = std::cmp::max(ans, dp[i][j]);
                }
            }
        }
        ans*ans
    }
}