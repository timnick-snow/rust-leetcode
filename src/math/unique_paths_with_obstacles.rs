#![allow(dead_code)]
/*
63. 不同路径 II
提示
中等
1.1K
相关企业
一个机器人位于一个 m x n 网格的左上角 （起始点在下图中标记为 “Start” ）。

机器人每次只能向下或者向右移动一步。机器人试图达到网格的右下角（在下图中标记为 “Finish”）。

现在考虑网格中有障碍物。那么从左上角到右下角将会有多少条不同的路径？

网格中的障碍物和空位置分别用 1 和 0 来表示。



示例 1：


输入：obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
输出：2
解释：3x3 网格的正中间有一个障碍物。
从左上角到右下角一共有 2 条不同的路径：
1. 向右 -> 向右 -> 向下 -> 向下
2. 向下 -> 向下 -> 向右 -> 向右
示例 2：


输入：obstacleGrid = [[0,1],[0,0]]
输出：1


提示：

m == obstacleGrid.length
n == obstacleGrid[i].length
1 <= m, n <= 100
obstacleGrid[i][j] 为 0 或 1
 */
struct Solution;

/*
动态规划

定义 dp[i][j]为走到(i,j)坐标的路径数量

(i,j)不是障碍物： dp[i][j] = dp[i-1][j] + dp[i][j-1]
(i,j)是障碍物： dp[i][j] = 0

初始化
dp[0][0] = grip[0][0]==0?1:0
dp[0][j] = 1 或 0，路途中有任何障碍物则为0
dp[i][0] = 1 或 0，路途中有任何障碍物则为0
 */
impl Solution {
    pub fn unique_paths_with_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; n]; m];
        // 初始化
        if grid[0][0] == 0 {
            dp[0][0] = 1;
        }
        for i in 0..m {
            if grid[i][0] == 1 {
                break;
            }
            dp[i][0] = 1;
        }
        for j in 0..n {
            if grid[0][j] == 1 {
                break;
            }
            dp[0][j] = 1;
        }
        // 状态计算
        for i in 1..m {
            for j in 1..n {
                if grid[i][j] == 1 {
                    dp[i][j] = 0;
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }
        dp[m - 1][n - 1]
    }
}


